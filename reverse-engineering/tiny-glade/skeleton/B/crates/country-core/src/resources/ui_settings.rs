// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/ui_settings.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 046752539f8564da3a2ffefff7fccfce20f64158c049a42226bf5acda0cfdfe6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_settings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 39, 155, 191, 197, 240, 511, 546, 577, 1600, 1627, 1643
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214A
// 首条位置: {"module":155,"inlinee":8522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":155,"record_offset":137420,"parent_record_offset":137396,"ancestor_record_offset":137396,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":155,"record_offset":140820,"parent_record_offset":140796,"ancestor_record_offset":140796,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":374792,"parent_record_offset":374768,"ancestor_record_offset":374768,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1600,"record_offset":374816,"parent_record_offset":374792,"ancestor_record_offset":374768,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":19244,"rva":979216,"va":5369688336}
// 编译实例 RVAs: 0xef110, 0xef290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t41792266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369688720,"raw_hex":"e80ab37d02","size":5,"soft_fail":false,"target_va":5411481152,"va":5369688881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42158477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369688336,"raw_hex":"e88d498302","size":5,"soft_fail":false,"target_va":5411847152,"va":5369688670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":19436,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":39,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":19244,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":39,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":19244,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":39,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":19244,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":22104,"rva":10775632,"va":5379484752}
// 编译实例 RVAs: 0xa46c50, 0xa47020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32189779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379485728,"raw_hex":"e8532deb01","size":5,"soft_fail":false,"target_va":5411675680,"va":5379485896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32362061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379484752,"raw_hex":"e84dceed01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379485086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":546,"record_offset":22296,"parent_record_offset":22104,"ancestor_record_offset":22104,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":546,"record_offset":22320,"parent_record_offset":22296,"ancestor_record_offset":22104,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":546,"record_offset":22344,"parent_record_offset":22320,"ancestor_record_offset":22104,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":546,"record_offset":22368,"parent_record_offset":22344,"ancestor_record_offset":22104,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33F3
// 首条位置: {"module":155,"inlinee":13299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":404392,"parent_record_offset":404368,"ancestor_record_offset":404368,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136364,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":6564,"source_line":1886,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":136784,"parent_record_offset":136392,"ancestor_record_offset":136336,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":9460,"parent_record_offset":9424,"ancestor_record_offset":9400,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":9524,"parent_record_offset":9424,"ancestor_record_offset":9400,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":9496,"parent_record_offset":9424,"ancestor_record_offset":9400,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136612,"parent_record_offset":136472,"ancestor_record_offset":136336,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136812,"parent_record_offset":136392,"ancestor_record_offset":136336,"inlinee":6496,"source_line":367,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136472,"parent_record_offset":136392,"ancestor_record_offset":136336,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136512,"parent_record_offset":136472,"ancestor_record_offset":136336,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":404416,"parent_record_offset":404392,"ancestor_record_offset":404368,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":136392,"parent_record_offset":136364,"ancestor_record_offset":136336,"inlinee":6565,"source_line":1511,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":136636,"parent_record_offset":136612,"ancestor_record_offset":136336,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":136540,"parent_record_offset":136512,"ancestor_record_offset":136336,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":136660,"parent_record_offset":136636,"ancestor_record_offset":136336,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":136568,"parent_record_offset":136540,"ancestor_record_offset":136336,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A6F
// 首条位置: {"module":546,"inlinee":31343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x345C
// 首条位置: {"module":155,"inlinee":13404}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":155,"record_offset":136872,"parent_record_offset":135812,"ancestor_record_offset":135812,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":136808,"parent_record_offset":135812,"ancestor_record_offset":135812,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":408280,"parent_record_offset":403592,"ancestor_record_offset":403592,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":135912,"parent_record_offset":135812,"ancestor_record_offset":135812,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":136780,"parent_record_offset":135812,"ancestor_record_offset":135812,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":404536,"parent_record_offset":404416,"ancestor_record_offset":403592,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":136156,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":136284,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":136704,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":136116,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":136664,"parent_record_offset":136564,"ancestor_record_offset":135812,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":155,"record_offset":137396,"parent_record_offset":137372,"ancestor_record_offset":135812,"inlinee":8522,"source_line":5,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":155,"record_offset":136252,"parent_record_offset":136228,"ancestor_record_offset":135812,"inlinee":13299,"source_line":5,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":155,"record_offset":136188,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":136564,"parent_record_offset":136536,"ancestor_record_offset":135812,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":135952,"parent_record_offset":135912,"ancestor_record_offset":135812,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":137204,"parent_record_offset":137168,"ancestor_record_offset":135812,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":155,"record_offset":137420,"parent_record_offset":137396,"ancestor_record_offset":135812,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":136636,"parent_record_offset":136564,"ancestor_record_offset":135812,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":136228,"parent_record_offset":135952,"ancestor_record_offset":135812,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":405572,"parent_record_offset":405548,"ancestor_record_offset":403592,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":403944,"parent_record_offset":403772,"ancestor_record_offset":403592,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":404736,"parent_record_offset":404692,"ancestor_record_offset":403592,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":403772,"parent_record_offset":403692,"ancestor_record_offset":403592,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":403832,"parent_record_offset":403772,"ancestor_record_offset":403592,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":403692,"parent_record_offset":403644,"ancestor_record_offset":403592,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":404692,"parent_record_offset":404652,"ancestor_record_offset":403592,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":404416,"parent_record_offset":404392,"ancestor_record_offset":403592,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":404392,"parent_record_offset":404368,"ancestor_record_offset":403592,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":403968,"parent_record_offset":403944,"ancestor_record_offset":403592,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":403864,"parent_record_offset":403832,"ancestor_record_offset":403592,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":403992,"parent_record_offset":403968,"ancestor_record_offset":403592,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":403896,"parent_record_offset":403864,"ancestor_record_offset":403592,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BD9
// 首条位置: {"module":1600,"inlinee":72665}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":408464,"parent_record_offset":408396,"ancestor_record_offset":408396,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":408436,"parent_record_offset":408396,"ancestor_record_offset":408396,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":409184,"parent_record_offset":408524,"ancestor_record_offset":408396,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":408756,"parent_record_offset":408604,"ancestor_record_offset":408396,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":408604,"parent_record_offset":408524,"ancestor_record_offset":408396,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":408652,"parent_record_offset":408604,"ancestor_record_offset":408396,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":408524,"parent_record_offset":408492,"ancestor_record_offset":408396,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":408780,"parent_record_offset":408756,"ancestor_record_offset":408396,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":408680,"parent_record_offset":408652,"ancestor_record_offset":408396,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":408804,"parent_record_offset":408780,"ancestor_record_offset":408396,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":408708,"parent_record_offset":408680,"ancestor_record_offset":408396,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":9324,"rva":976320,"va":5369685440}
// 编译实例 RVAs: 0xee5c0, 0xee880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140349e00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2471205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369686144,"raw_hex":"e825b52500","size":5,"soft_fail":false,"target_va":5372157440,"va":5369686230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14034a130
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2472058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369686144,"raw_hex":"e87ab82500","size":5,"soft_fail":false,"target_va":5372158256,"va":5369686193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14034ca00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2483030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685440,"raw_hex":"e856e32500","size":5,"soft_fail":false,"target_va":5372168704,"va":5369685669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14034e490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2489932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685440,"raw_hex":"e84cfe2500","size":5,"soft_fail":false,"target_va":5372175504,"va":5369685567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14034f7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2494761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685440,"raw_hex":"e829112600","size":5,"soft_fail":false,"target_va":5372180384,"va":5369685618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140351a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2503298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685440,"raw_hex":"e882322600","size":5,"soft_fail":false,"target_va":5372189216,"va":5369685913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140352450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2506315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685440,"raw_hex":"e84b3e2600","size":5,"soft_fail":false,"target_va":5372191824,"va":5369685504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":9836,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":39,"record_offset":9544,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":39,"record_offset":9776,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":39,"record_offset":10228,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":5560,"source_line":854,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":9888,"parent_record_offset":9860,"ancestor_record_offset":9324,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":39,"record_offset":10256,"parent_record_offset":10228,"ancestor_record_offset":10016,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":39,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":9324,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":8156,"rva":975920,"va":5369685040}
// 编译实例 RVAs: 0xee430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1864435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685040,"raw_hex":"e8f3721c00","size":5,"soft_fail":false,"target_va":5371549808,"va":5369685368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3989195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685040,"raw_hex":"e8cbde3c00","size":5,"soft_fail":false,"target_va":5373674384,"va":5369685184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3995651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685040,"raw_hex":"e803f83c00","size":5,"soft_fail":false,"target_va":5373680768,"va":5369685112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bedb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4000015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685040,"raw_hex":"e80f093d00","size":5,"soft_fail":false,"target_va":5373685168,"va":5369685148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41632822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369685040,"raw_hex":"e836447b02","size":5,"soft_fail":false,"target_va":5411318240,"va":5369685413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":8436,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":9172,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":8976,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8828,"parent_record_offset":8796,"ancestor_record_offset":8156,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8696,"parent_record_offset":8668,"ancestor_record_offset":8156,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8756,"parent_record_offset":8724,"ancestor_record_offset":8156,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8156,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":8156,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":8724,"parent_record_offset":8696,"ancestor_record_offset":8156,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":8156,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":8796,"parent_record_offset":8632,"ancestor_record_offset":8156,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":8576,"parent_record_offset":8548,"ancestor_record_offset":8156,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":8916,"parent_record_offset":8632,"ancestor_record_offset":8156,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":8668,"parent_record_offset":8632,"ancestor_record_offset":8156,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":39,"record_offset":8604,"parent_record_offset":8576,"ancestor_record_offset":8156,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":8492,"parent_record_offset":8464,"ancestor_record_offset":8156,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":8156,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":8464,"parent_record_offset":8436,"ancestor_record_offset":8156,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":39,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":8156,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":8548,"parent_record_offset":8520,"ancestor_record_offset":8156,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":39,"record_offset":8520,"parent_record_offset":8492,"ancestor_record_offset":8156,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":21080,"rva":10775248,"va":5379484368}
// 编译实例 RVAs: 0xa46ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141583370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11782219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379484368,"raw_hex":"e84bc8b300","size":5,"soft_fail":false,"target_va":5391266672,"va":5379484448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588730
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11803587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379484368,"raw_hex":"e8c31bb400","size":5,"soft_fail":false,"target_va":5391288112,"va":5379484520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158c770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11820071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379484368,"raw_hex":"e8275cb400","size":5,"soft_fail":false,"target_va":5391304560,"va":5379484484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31862310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379484368,"raw_hex":"e8262ee601","size":5,"soft_fail":false,"target_va":5411347040,"va":5379484725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140a44bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-8295","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379484368,"raw_hex":"e999dfffff","size":5,"soft_fail":false,"target_va":5379476400,"va":5379484690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":546,"record_offset":21356,"parent_record_offset":21080,"ancestor_record_offset":21080,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":546,"record_offset":21904,"parent_record_offset":21080,"ancestor_record_offset":21080,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21756,"parent_record_offset":21724,"ancestor_record_offset":21080,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21624,"parent_record_offset":21596,"ancestor_record_offset":21080,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21684,"parent_record_offset":21652,"ancestor_record_offset":21080,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21804,"parent_record_offset":21780,"ancestor_record_offset":21080,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21780,"parent_record_offset":21756,"ancestor_record_offset":21080,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":546,"record_offset":21652,"parent_record_offset":21624,"ancestor_record_offset":21080,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":21556,"parent_record_offset":21528,"ancestor_record_offset":21080,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":21724,"parent_record_offset":21556,"ancestor_record_offset":21080,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":21500,"parent_record_offset":21472,"ancestor_record_offset":21080,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":21844,"parent_record_offset":21556,"ancestor_record_offset":21080,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":21596,"parent_record_offset":21556,"ancestor_record_offset":21080,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":546,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":21080,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":546,"record_offset":21412,"parent_record_offset":21384,"ancestor_record_offset":21080,"inlinee":6920,"source_line":2007,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":546,"record_offset":21384,"parent_record_offset":21356,"ancestor_record_offset":21080,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":546,"record_offset":21472,"parent_record_offset":21444,"ancestor_record_offset":21080,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":546,"record_offset":21444,"parent_record_offset":21412,"ancestor_record_offset":21080,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38FE
// 首条位置: {"module":197,"inlinee":14590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$4::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":71192,"rva":11611920,"va":5380321040}
// 编译实例 RVAs: 0xb12f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A6E
// 首条位置: {"module":546,"inlinee":31342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2153
// 首条位置: {"module":155,"inlinee":8531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":79288,"parent_record_offset":79232,"ancestor_record_offset":79232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":79260,"parent_record_offset":79232,"ancestor_record_offset":79232,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":24188,"parent_record_offset":24040,"ancestor_record_offset":23896,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":24000,"parent_record_offset":23956,"ancestor_record_offset":23896,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":24040,"parent_record_offset":24000,"ancestor_record_offset":23896,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":24084,"parent_record_offset":24040,"ancestor_record_offset":23896,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23956,"parent_record_offset":23924,"ancestor_record_offset":23896,"inlinee":5760,"source_line":2020,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":23708,"inlinee":5763,"source_line":2073,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":24212,"parent_record_offset":24188,"ancestor_record_offset":23896,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":23896,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":24236,"parent_record_offset":24212,"ancestor_record_offset":23896,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":24140,"parent_record_offset":24112,"ancestor_record_offset":23896,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":23132,"rva":10776016,"va":5379485136}
// 编译实例 RVAs: 0xa46dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cc30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10247644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379485136,"raw_hex":"e8dc5d9c00","size":5,"soft_fail":false,"target_va":5389732912,"va":5379485263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140d9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10250856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379485136,"raw_hex":"e8686a9c00","size":5,"soft_fail":false,"target_va":5389736352,"va":5379485491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31824609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379485136,"raw_hex":"e8e19ae501","size":5,"soft_fail":false,"target_va":5411309856,"va":5379485242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31825911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379485136,"raw_hex":"e8f79fe501","size":5,"soft_fail":false,"target_va":5411311520,"va":5379485604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23348,"parent_record_offset":23132,"ancestor_record_offset":23132,"inlinee":5750,"source_line":1855,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":546,"record_offset":23708,"parent_record_offset":23348,"ancestor_record_offset":23132,"inlinee":8531,"source_line":31,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":546,"record_offset":23744,"parent_record_offset":23708,"ancestor_record_offset":23132,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23620,"parent_record_offset":23468,"ancestor_record_offset":23132,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":24000,"parent_record_offset":23956,"ancestor_record_offset":23132,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23468,"parent_record_offset":23348,"ancestor_record_offset":23132,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23520,"parent_record_offset":23468,"ancestor_record_offset":23132,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23956,"parent_record_offset":23924,"ancestor_record_offset":23132,"inlinee":5760,"source_line":2020,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":546,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":23132,"inlinee":5763,"source_line":2073,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":23132,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":23548,"parent_record_offset":23520,"ancestor_record_offset":23132,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":23132,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":546,"record_offset":23576,"parent_record_offset":23548,"ancestor_record_offset":23132,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":71324,"rva":9148896,"va":5377858016}
// 编译实例 RVAs: 0x8b99e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::previous
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":71456,"rva":11611952,"va":5380321072}
// 编译实例 RVAs: 0xb12f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2149
// 首条位置: {"module":191,"inlinee":8521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":191,"record_offset":218100,"parent_record_offset":218080,"ancestor_record_offset":218080,"inlinee":5432,"source_line":249,"archive":"_pdb/B/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":191,"record_offset":218124,"parent_record_offset":218100,"ancestor_record_offset":218080,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":15732,"rva":11591792,"va":5380300912}
// 编译实例 RVAs: 0xb0e070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":15880,"rva":11591808,"va":5380300928}
// 编译实例 RVAs: 0xb0e080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t586666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380300928,"raw_hex":"e8aaf30800","size":5,"soft_fail":false,"target_va":5380887680,"va":5380301009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2591399","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380300928,"raw_hex":"e9a78a2700","size":5,"soft_fail":false,"target_va":5382892432,"va":5380301028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: ui_settings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::ToolbarAlignment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >", "constructor": "toolbar::mei::widgets::tweaker::impl$13::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::UiSettings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_settings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_settings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::playermade_stairs_audio::StairsAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::playermade_stairs_audio::StairsAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::playermade_stairs_audio::StairsAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
// 信息 ID: b224bf220795c72c371500ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::toolbar_setup @ 0x2346f5a -> 0xb0e070 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_startup::toolbar_setup @ 0x2346fb4 -> 0xb0e070 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar @ 0x2625bb1 -> 0xb0e070 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar @ 0x2625c01 -> 0xb0e070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize
// 信息 ID: ffd4093ddcde0e04736d0a3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=137396; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_config::_::impl$0::deserialize::__Visitor>; module=155, inline_record=140796; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=371824; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=374768; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=378028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 520efcf9546b67c8ad1c3383
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor>; module=546, inline_record=4608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 2924028330da859838f3ba88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x3a00a9 -> 0xef110 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x4a757a -> 0xef110 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x4a75b0 -> 0xef110 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> > @ 0x37f446 -> 0xef290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 2b5d6dfbb947338653644024
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x142ed77 -> 0xa46c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<country_core::resources::ui_settings::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > > @ 0x140ccf7 -> 0xa47020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 2ae97fa29872c750d81af4b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> >; module=155, inline_record=9400; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=136252; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=136416; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=136536; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$5::deserialize<country_core::resources::ui_settings::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1600, inline_record=136336; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=404368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: b836611815312392c5e6e8ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=546, inline_record=4816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 878a3470b31db4d8402502f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=79232; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=546, inline_record=23708; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=546, inline_record=23896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 5e798d1ab8f4f77eeeccb20f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=135812; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=403592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 1251eeea6bf81b275f3bf051
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=408396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 信息 ID: ff46f74ed17b83495e6aba5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::StrRead>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x142f340 -> 0xa46dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::next
// 信息 ID: 1e2b81d05e3d781c29aa2d5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfc1a1 -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfe7a0 -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$0::closure_env$0> > @ 0x2ba6df -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$0::closure_env$0> > @ 0x2ba7e2 -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f3ea -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f6cb -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32f99a -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x32fc7b -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x33053a -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x33081b -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x33318a -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x33346b -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x333d2a -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x33400b -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6bc2 -> 0x8b99e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::previous
// 信息 ID: c2889971b6289175d390355d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f48b -> 0xb12f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f407 -> 0xb12f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize
// 信息 ID: e6bf986d56c7f210b80cfe46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>; module=191, inline_record=218080; 不是独立调用边。
// [PDB 内联宿主] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>; module=1627, inline_record=294008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 4f89a6c831f57ba1de4c727d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings> @ 0x34ff62 -> 0xee5c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment> @ 0x34f9a2 -> 0xee880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: b9892f4cad45d129779fb242
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x4be458 -> 0xee430 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: ef505aa1b1d735e17801a6df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x158c4d4 -> 0xa46ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$1::fmt
// 信息 ID: 429001f4dff53a4c9ab521e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x32f4e3 -> 0xb0e080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$3::clone
// 信息 ID: c6bfdc8993179642cab7f87c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=10144; 不是独立调用边。
// [PDB 内联宿主] country_core::game_config::impl$11::clone; module=240, inline_record=14808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=511, inline_record=42596; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=511, inline_record=43396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$4::default
// 信息 ID: 40a7d33fe5f029caa3b57a32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x3a0650 -> 0xb12f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
