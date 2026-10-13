// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset-pipe/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 00f154b4d4dbbf48f09afc028063794e370ae6586b69363696c37bf6b6e0b3c0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 687, 1667, 1668, 1845, 1954, 3254, 4416, 4417, 4418, 4926, 5061
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use ddsfile; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"ddsfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"},{"name":"enum_primitive","source":"PDB_registry_path_only","version":"0.1.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use num_traits; // namespace candidate
//   package identities: [{"name":"num-traits","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.19"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resolve_opts
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":687,"record_offset":264,"rva":14249312,"va":5382958432}
// 编译实例 RVAs: 0xd96d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33892767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382958432,"raw_hex":"e89f290502","size":5,"soft_fail":false,"target_va":5416851728,"va":5382958956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":687,"record_offset":1176,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":687,"record_offset":396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":516,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":687,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":924,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1040,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1120,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":1080,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":884,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":687,"record_offset":748,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resolve_opts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":264,"inlinee":45630,"source_line":268,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resolve_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":687,"record_offset":648,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":45629,"source_line":269,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":696,"parent_record_offset":648,"ancestor_record_offset":264,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":264,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":687,"record_offset":972,"parent_record_offset":924,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":687,"record_offset":844,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":687,"record_offset":604,"parent_record_offset":568,"ancestor_record_offset":264,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":687,"record_offset":568,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture_processing::process_dds_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1667,"record_offset":1088,"rva":24978544,"va":5393687664}
// 编译实例 RVAs: 0x17d2470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9079011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e81d7775ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393688414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18010242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e87e2fedfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393688925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18009746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e86e31edfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393688445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24966725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8bb0983fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393688912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d6da0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8e8410000","size":5,"soft_fail":false,"target_va":5393706400,"va":5393689523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425939c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14423267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8e314dc00","size":5,"soft_fail":false,"target_va":5408111040,"va":5393687768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142594250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14425437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e85d1ddc00","size":5,"soft_fail":false,"target_va":5408113232,"va":5393687790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23369195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8eb956401","size":5,"soft_fail":false,"target_va":5417057056,"va":5393687856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8893223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8d94c78ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5393688082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22931479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e817e85d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393689310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23162881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e801706101","size":5,"soft_fail":false,"target_va":5416851728,"va":5393688842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23095805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8fd696001","size":5,"soft_fail":false,"target_va":5416785104,"va":5393689294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22933522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e812f05d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393688713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22932418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8c2eb5d01","size":5,"soft_fail":false,"target_va":5416621632,"va":5393689209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_d3d_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5842639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8cf265900","size":5,"soft_fail":false,"target_va":5399530672,"va":5393688028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5843543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e8572a5900","size":5,"soft_fail":false,"target_va":5399532016,"va":5393688468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_dxgi_format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t22082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393687664,"raw_hex":"e842560000","size":5,"soft_fail":false,"target_va":5393710704,"va":5393688617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2628,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":1667,"record_offset":2508,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1667,"record_offset":1460,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1667,"record_offset":2536,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1652,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":2020,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":2304,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":1308,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1667,"record_offset":1348,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":1516,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":42296,"source_line":249,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_dxgi_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":1572,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":42293,"source_line":218,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_num_mipmap_levels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":1667,"record_offset":2472,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":82459,"source_line":322,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":1908,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1376,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1667,"record_offset":1280,"parent_record_offset":1088,"ancestor_record_offset":1088,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":1088,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1088,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1088,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":1088,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1667,"record_offset":2172,"parent_record_offset":2116,"ancestor_record_offset":1088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1088,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1667,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1088,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2776,"parent_record_offset":2748,"ancestor_record_offset":1088,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2836,"parent_record_offset":2804,"ancestor_record_offset":1088,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":1088,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1667,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2656,"parent_record_offset":2628,"ancestor_record_offset":1088,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":1088,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1667,"record_offset":2712,"parent_record_offset":2684,"ancestor_record_offset":1088,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1667,"record_offset":2748,"parent_record_offset":2712,"ancestor_record_offset":1088,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::process_dds_image::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":1088,"inlinee":82457,"source_line":538,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture_processing::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture_processing.rs
//     evidence={"module":1667,"record_offset":1792,"parent_record_offset":1652,"ancestor_record_offset":1088,"inlinee":82458,"source_line":539,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$138::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":1667,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1088,"inlinee":10438,"source_line":296,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1088,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1088,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1667,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1088,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1667,"record_offset":2996,"parent_record_offset":2712,"ancestor_record_offset":1088,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1088,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1667,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":1088,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1667,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":1088,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::format::dxgi::DxgiFormat::try_from_pixel_format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ddsfile-0.5.1/src/format/dxgi.rs
//     evidence={"module":1667,"record_offset":1604,"parent_record_offset":1572,"ancestor_record_offset":1088,"inlinee":42295,"source_line":416,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":1088,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1667,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1088,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1667,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1088,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01667.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ddsfile::Dds::read<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3254,"record_offset":264,"rva":39401920,"va":5408111040}
// 编译实例 RVAs: 0x25939c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..io..cursor..Cursor$LT$T$GT$$u20$as$u20$std..io..Read$GT$::read_to_end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4944743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408111040,"raw_hex":"e867734b00","size":5,"soft_fail":false,"target_va":5413056304,"va":5408111556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-32432920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408111040,"raw_hex":"e8e81c11fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5408111619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a4b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4945413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408111040,"raw_hex":"e805764b00","size":5,"soft_fail":false,"target_va":5413056592,"va":5408111174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a4b590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4946479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408111040,"raw_hex":"e82f7a4b00","size":5,"soft_fail":false,"target_va":5413057936,"va":5408111452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3254,"record_offset":968,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":3254,"record_offset":448,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3254,"record_offset":904,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3254,"record_offset":1080,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3254,"record_offset":840,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3254,"record_offset":812,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_direct] std::io::impls::impl$0::read_to_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":3254,"record_offset":996,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":94580,"source_line":35,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3254,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3254,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3254,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3254,"record_offset":1208,"parent_record_offset":1152,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3254,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3254,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3254,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3254,"record_offset":568,"parent_record_offset":532,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3254,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3254,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3254,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":264,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3254,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":264,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3254,"record_offset":596,"parent_record_offset":532,"ancestor_record_offset":264,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3254,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3254,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3254,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":264,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3254,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":3254,"record_offset":476,"parent_record_offset":448,"ancestor_record_offset":264,"inlinee":73027,"source_line":45,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":3254,"record_offset":684,"parent_record_offset":504,"ancestor_record_offset":264,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/03254.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ddsfile::format::pixel_format::PixelFormat::read<ref_mut$<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4926,"record_offset":264,"rva":45703904,"va":5414413024}
// 编译实例 RVAs: 0x2b962e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-38734501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414413024,"raw_hex":"e85bf5b0fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5414413184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45690982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414413024,"raw_hex":"e89acf46fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5414413169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2207041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414413024,"raw_hex":"e841ad2100","size":5,"soft_fail":false,"target_va":5416620794,"va":5414413748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4926,"record_offset":1380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4926,"record_offset":484,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4926,"record_offset":896,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4926,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4926,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4926,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4926,"record_offset":1644,"parent_record_offset":1616,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4926,"record_offset":1556,"parent_record_offset":1528,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4926,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4926,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4926,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":264,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4926,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":264,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4926,"record_offset":1492,"parent_record_offset":1464,"ancestor_record_offset":264,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4926,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4926,"record_offset":1528,"parent_record_offset":1492,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4926,"record_offset":644,"parent_record_offset":612,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4926,"record_offset":1772,"parent_record_offset":1492,"ancestor_record_offset":264,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4926,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4926,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":264,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4926,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4926,"record_offset":672,"parent_record_offset":612,"ancestor_record_offset":264,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4926,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4926,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4926,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":264,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4926,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":264,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_exact
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4926,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":264,"inlinee":73027,"source_line":45,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4926,"record_offset":760,"parent_record_offset":584,"ancestor_record_offset":264,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/04926.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ddsfile::header10::Header10::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4418,"record_offset":768,"rva":44348816,"va":5413057936}
// 编译实例 RVAs: 0x2a4b590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d6da0
//     owner=unresolved_namespace; count=6
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19352009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413057936,"raw_hex":"e837b6d8fe","size":5,"soft_fail":false,"target_va":5393706400,"va":5413058404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426ef710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3522444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413057936,"raw_hex":"e87440caff","size":5,"soft_fail":false,"target_va":5409535760,"va":5413058199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426ef8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3521980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413057936,"raw_hex":"e84442caff","size":5,"soft_fail":false,"target_va":5409536208,"va":5413058183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::header10::impl$2::read::closure$1<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413057936,"raw_hex":"e8fa010000","size":5,"soft_fail":false,"target_va":5413058752,"va":5413058241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::header10::impl$2::read::closure$2<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413057936,"raw_hex":"e85c010000","size":5,"soft_fail":false,"target_va":5413058640,"va":5413058287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4418,"record_offset":972,"parent_record_offset":768,"ancestor_record_offset":768,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4418,"record_offset":2920,"parent_record_offset":768,"ancestor_record_offset":768,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4418,"record_offset":3136,"parent_record_offset":768,"ancestor_record_offset":768,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4418,"record_offset":2864,"parent_record_offset":768,"ancestor_record_offset":768,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_direct] num_traits::cast::FromPrimitive::from_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"num-traits","version":"0.2.19","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-traits-0.2.19/src/cast.rs
//     evidence={"module":4418,"record_offset":2892,"parent_record_offset":768,"ancestor_record_offset":768,"inlinee":15383,"source_line":454,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4418,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":768,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4418,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":768,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4418,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":768,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4418,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":768,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4418,"record_offset":1128,"parent_record_offset":1072,"ancestor_record_offset":768,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4418,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":768,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4418,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":768,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::header10::impl$11::from_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum_primitive","version":"0.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum_primitive-0.1.1/src/lib.rs
//     evidence={"module":4418,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":768,"inlinee":128794,"source_line":66,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::header10::impl$47::from_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum_primitive","version":"0.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum_primitive-0.1.1/src/lib.rs
//     evidence={"module":4418,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":768,"inlinee":128796,"source_line":66,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4418,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":768,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4418,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":768,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_exact
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4418,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":768,"inlinee":73027,"source_line":45,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4418,"record_offset":1216,"parent_record_offset":1044,"ancestor_record_offset":768,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/04418.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ddsfile::header::Header::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4417,"record_offset":264,"rva":44347472,"va":5413056592}
// 编译实例 RVAs: 0x2a4b050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-37378393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413056592,"raw_hex":"e8a7a6c5fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413057076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44334874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413056592,"raw_hex":"e8e6805bfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413057061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b962e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1355788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413056592,"raw_hex":"e80cb01400","size":5,"soft_fail":false,"target_va":5414413024,"va":5413057231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3562871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413056592,"raw_hex":"e8775d3600","size":5,"soft_fail":false,"target_va":5416620794,"va":5413057918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4417,"record_offset":3628,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4417,"record_offset":472,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32_into
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4417,"record_offset":3196,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":128785,"source_line":597,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4417,"record_offset":4096,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4417,"record_offset":864,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::header::Caps2::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4417,"record_offset":6344,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":128789,"source_line":563,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::header::Caps::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4417,"record_offset":6312,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":128787,"source_line":563,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4417,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4417,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4417,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4417,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4417,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4417,"record_offset":3864,"parent_record_offset":3832,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4417,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4417,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":264,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4417,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":264,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4417,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":264,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4417,"record_offset":3656,"parent_record_offset":3628,"ancestor_record_offset":264,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4417,"record_offset":3776,"parent_record_offset":3740,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4417,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4417,"record_offset":4020,"parent_record_offset":3740,"ancestor_record_offset":264,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4417,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4417,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4417,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":264,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4417,"record_offset":644,"parent_record_offset":584,"ancestor_record_offset":264,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4417,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4417,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":264,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4417,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":264,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":4417,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_exact
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4417,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":73027,"source_line":45,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":4417,"record_offset":732,"parent_record_offset":556,"ancestor_record_offset":264,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/04417.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::cache_root
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1954,"record_offset":264,"rva":30665504,"va":5399374624}
// 编译实例 RVAs: 0x1d3eb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14581435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399374624,"raw_hex":"e8458121ff","size":5,"soft_fail":false,"target_va":5384793264,"va":5399374694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14718523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399374624,"raw_hex":"e8c5691fff","size":5,"soft_fail":false,"target_va":5384656240,"va":5399374758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23696084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399374624,"raw_hex":"e82c6d96fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399374783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14750629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399374624,"raw_hex":"e85bec1eff","size":5,"soft_fail":false,"target_va":5384624080,"va":5399374704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16235829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399374624,"raw_hex":"e8cb4208ff","size":5,"soft_fail":false,"target_va":5383138832,"va":5399374656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1954,"record_offset":616,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1954,"record_offset":380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1954,"record_offset":588,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1954,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1954,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1954,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1954,"record_offset":840,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1954,"record_offset":812,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1954,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1954,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::cache_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/lib.rs
//     evidence={"module":1954,"record_offset":416,"parent_record_offset":380,"ancestor_record_offset":264,"inlinee":94006,"source_line":19,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1954,"record_offset":528,"parent_record_offset":416,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1954,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":264,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1954,"record_offset":444,"parent_record_offset":416,"ancestor_record_offset":264,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1954,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::cache_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16F36
// 首条位置: {"module":1954,"inlinee":94006}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1954,"record_offset":528,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1954,"record_offset":444,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1954,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":416,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/01954.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1954,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":416,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/01954.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset_pipe::cache_root
// 信息 ID: 1677dd535d5b2947cdd2a25e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_pack_pipe::clutter_pack_package_dir @ 0xce50da -> 0x1d3eb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_pack_pipe::mod_icons::mod_clutter_icon_package_dir @ 0xce98aa -> 0x1d3eb20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::store::open @ 0x1806cc8 -> 0x1d3eb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::cache_root::closure$0
// 信息 ID: 67b2d97dffdfb1a0182b6671
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::cache_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::cache_root; module=1954, inline_record=416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resolve_opts
// 信息 ID: 2446cd25daab6f18f56def64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::build_texture_assets::async_fn$0 @ 0x16f192b -> 0xd96d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258d315 -> 0xd96d60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a432c4 -> 0xd96d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture_processing::process_dds_image
// 信息 ID: b3e5746ba9911804a1b682c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<asset::texture::blob::TextureAssetBlob,anyhow::Error,enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x8fe6d5 -> 0x17d2470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ddsfile::Dds::read<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > >
// 信息 ID: d7a894a501ef40a6616db820
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["std::io::cursor::Cursor<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] ddsfile::Dds::read -> ["ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture_processing::process_dds_image @ 0x17d24d8 -> 0x25939c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ddsfile::format::pixel_format::PixelFormat::read<ref_mut$<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > > >
// 信息 ID: 0ff07513f59ec76f1756aa4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["std::io::cursor::Cursor<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > >"]; 字段布局 = unknown。
// [类型关系] ddsfile::format::pixel_format::PixelFormat::read -> ["ref_mut$<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ddsfile::header::Header::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > > @ 0x2a4b2cf -> 0x2b962e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ddsfile::header10::Header10::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
// 信息 ID: c6a35b3b0cdf12c19ac4f3c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["std::io::cursor::Cursor<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// [类型关系] ddsfile::header10::Header10::read -> ["ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ddsfile::Dds::read<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > @ 0x2593b5c -> 0x2a4b590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ddsfile::header::Header::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > >
// 信息 ID: 1b311555de2df2f4cb327ca1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["std::io::cursor::Cursor<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// [类型关系] ddsfile::header::Header::read -> ["ref_mut$<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ddsfile::Dds::read<ref_mut$<std::io::cursor::Cursor<ref$<slice2$<u8> > > > > @ 0x2593a46 -> 0x2a4b050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
