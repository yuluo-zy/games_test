// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/logging.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9ef8e48c287797a461c49addf0978e78b32528aae2a0ce4582959298b944e76d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: logging.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 117
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use colored; // namespace candidate
//   package identities: [{"name":"colored","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.3"}]
// use core; // namespace candidate
//   package identities: []
// use fern; // namespace candidate
//   package identities: [{"name":"fern","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.1"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_date_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FD4
// 首条位置: {"module":117,"inlinee":12244}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":73484,"parent_record_offset":73404,"ancestor_record_offset":73404,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":117,"record_offset":73456,"parent_record_offset":73404,"ancestor_record_offset":73404,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":73520,"parent_record_offset":73484,"ancestor_record_offset":73404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FCF
// 首条位置: {"module":117,"inlinee":12239}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":72468,"parent_record_offset":72388,"ancestor_record_offset":72388,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":117,"record_offset":72440,"parent_record_offset":72388,"ancestor_record_offset":72388,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":72508,"parent_record_offset":72468,"ancestor_record_offset":72388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":117,"record_offset":69436,"rva":3032928,"va":5371742048}
// 编译实例 RVAs: 0x2e4760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3156037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e845283000","size":5,"soft_fail":false,"target_va":5374900736,"va":5371744694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8330
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3158732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8cc323000","size":5,"soft_fail":false,"target_va":5374903088,"va":5371744351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3339256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8f8f33200","size":5,"soft_fail":false,"target_va":5375081488,"va":5371742227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3338054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e846ef3200","size":5,"soft_fail":false,"target_va":5375081504,"va":5371743445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11084329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e82922a900","size":5,"soft_fail":false,"target_va":5382827728,"va":5371743394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39574344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e848db5b02","size":5,"soft_fail":false,"target_va":5411318560,"va":5371744211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39796741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e805405f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5371744246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11096011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8cb4fa900","size":5,"soft_fail":false,"target_va":5382840208,"va":5371744192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2613878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e876e22700","size":5,"soft_fail":false,"target_va":5374356336,"va":5371742453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] colored::control::set_virtual_terminal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"colored","version":"1.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3766765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8ed793900","size":5,"soft_fail":false,"target_va":5375508896,"va":5371742126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<fern::builders::OutputInner> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e81cdeffff","size":5,"soft_fail":false,"target_va":5371735744,"va":5371744415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39798577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e831475f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5371744298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::builders::Dispatch::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3754992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8f04b3900","size":5,"soft_fail":false,"target_va":5375499072,"va":5371744075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::builders::Dispatch::level_for<ref$<str$> >
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8cee5ffff","size":5,"soft_fail":false,"target_va":5371736368,"va":5371743069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9146190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e84e8f8b00","size":5,"soft_fail":false,"target_va":5380889536,"va":5371743341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10965258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e80a51a700","size":5,"soft_fail":false,"target_va":5382708672,"va":5371743409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::stdout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10972213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8356ca700","size":5,"soft_fail":false,"target_va":5382714576,"va":5371742358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11000693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e875dba700","size":5,"soft_fail":false,"target_va":5382744080,"va":5371743382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10269802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e86ab49c00","size":5,"soft_fail":false,"target_va":5382011904,"va":5371742097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::utc_offset::UtcOffset::local_offset_at
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10269618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371742048,"raw_hex":"e8b2b39c00","size":5,"soft_fail":false,"target_va":5382011728,"va":5371742105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":72096,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":69608,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":71568,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::chain
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":117,"record_offset":69960,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12232,"source_line":198,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":117,"record_offset":69692,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12230,"source_line":171,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":117,"record_offset":69932,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12231,"source_line":224,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":117,"record_offset":69664,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12229,"source_line":127,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":117,"record_offset":69636,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12227,"source_line":158,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":117,"record_offset":70600,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12234,"source_line":1474,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":117,"record_offset":70516,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":117,"record_offset":70740,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":117,"record_offset":70656,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12235,"source_line":1453,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":117,"record_offset":70544,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":12233,"source_line":1380,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":117,"record_offset":70712,"parent_record_offset":69436,"ancestor_record_offset":69436,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":69828,"parent_record_offset":69804,"ancestor_record_offset":69436,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":69852,"parent_record_offset":69828,"ancestor_record_offset":69436,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":71188,"parent_record_offset":71164,"ancestor_record_offset":69436,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":69772,"parent_record_offset":69740,"ancestor_record_offset":69436,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":69804,"parent_record_offset":69772,"ancestor_record_offset":69436,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":71164,"parent_record_offset":71108,"ancestor_record_offset":69436,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":117,"record_offset":69740,"parent_record_offset":69692,"ancestor_record_offset":69436,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":72148,"parent_record_offset":72124,"ancestor_record_offset":69436,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":72196,"parent_record_offset":72172,"ancestor_record_offset":69436,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":70080,"parent_record_offset":70048,"ancestor_record_offset":69436,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":71136,"parent_record_offset":71108,"ancestor_record_offset":69436,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":71108,"parent_record_offset":71084,"ancestor_record_offset":69436,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":70128,"parent_record_offset":70104,"ancestor_record_offset":69436,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":70104,"parent_record_offset":70080,"ancestor_record_offset":69436,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":71084,"parent_record_offset":71060,"ancestor_record_offset":69436,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":117,"record_offset":70048,"parent_record_offset":70004,"ancestor_record_offset":69436,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":117,"record_offset":70004,"parent_record_offset":69960,"ancestor_record_offset":69436,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":117,"record_offset":72172,"parent_record_offset":72148,"ancestor_record_offset":69436,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":70940,"parent_record_offset":70740,"ancestor_record_offset":69436,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":69880,"parent_record_offset":69852,"ancestor_record_offset":69436,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":70168,"parent_record_offset":70004,"ancestor_record_offset":69436,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":117,"record_offset":70844,"parent_record_offset":70820,"ancestor_record_offset":69436,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":117,"record_offset":70892,"parent_record_offset":70868,"ancestor_record_offset":69436,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":117,"record_offset":70868,"parent_record_offset":70844,"ancestor_record_offset":69436,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":117,"record_offset":70820,"parent_record_offset":70796,"ancestor_record_offset":69436,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":117,"record_offset":70796,"parent_record_offset":70772,"ancestor_record_offset":69436,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":117,"record_offset":70772,"parent_record_offset":70740,"ancestor_record_offset":69436,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":117,"record_offset":70624,"parent_record_offset":70600,"ancestor_record_offset":69436,"inlinee":11597,"source_line":218,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":117,"record_offset":70680,"parent_record_offset":70656,"ancestor_record_offset":69436,"inlinee":11598,"source_line":215,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":117,"record_offset":70568,"parent_record_offset":70544,"ancestor_record_offset":69436,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::logging::set_up_logging::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":117,"record_offset":72124,"parent_record_offset":72096,"ancestor_record_offset":69436,"inlinee":12237,"source_line":123,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":117,"record_offset":72244,"rva":3036144,"va":5371745264}
// 编译实例 RVAs: 0x2e53f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14032c740
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t291517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e8bd720400","size":5,"soft_fail":false,"target_va":5372036928,"va":5371745406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3154779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e85b233000","size":5,"soft_fail":false,"target_va":5374900736,"va":5371745952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3335716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e824e63200","size":5,"soft_fail":false,"target_va":5375081504,"va":5371745783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39797021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e81d415f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5371745854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3756588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e82c523900","size":5,"soft_fail":false,"target_va":5375502352,"va":5371745759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10266586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e8daa79c00","size":5,"soft_fail":false,"target_va":5382011904,"va":5371745313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10266651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371745264,"raw_hex":"e81ba89c00","size":5,"soft_fail":false,"target_va":5382012016,"va":5371745360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":117,"record_offset":72720,"parent_record_offset":72244,"ancestor_record_offset":72244,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":72748,"parent_record_offset":72244,"ancestor_record_offset":72244,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":117,"record_offset":72644,"parent_record_offset":72244,"ancestor_record_offset":72244,"inlinee":12242,"source_line":230,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_time
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":117,"record_offset":72388,"parent_record_offset":72244,"ancestor_record_offset":72244,"inlinee":12239,"source_line":30,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":72924,"parent_record_offset":72900,"ancestor_record_offset":72244,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":72900,"parent_record_offset":72844,"ancestor_record_offset":72244,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":72872,"parent_record_offset":72844,"ancestor_record_offset":72244,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":72844,"parent_record_offset":72820,"ancestor_record_offset":72244,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":72820,"parent_record_offset":72796,"ancestor_record_offset":72244,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":72508,"parent_record_offset":72468,"ancestor_record_offset":72244,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":72468,"parent_record_offset":72388,"ancestor_record_offset":72244,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] fern::colors::ColoredLevelConfig::get_color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":117,"record_offset":72672,"parent_record_offset":72644,"ancestor_record_offset":72244,"inlinee":12243,"source_line":235,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":117,"record_offset":72440,"parent_record_offset":72388,"ancestor_record_offset":72244,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":117,"record_offset":73264,"rva":3036992,"va":5371746112}
// 编译实例 RVAs: 0x2e5740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14032c740
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t290671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e86f6f0400","size":5,"soft_fail":false,"target_va":5372036928,"va":5371746252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3154029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e86d203000","size":5,"soft_fail":false,"target_va":5374900736,"va":5371746702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3334910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e8fee23200","size":5,"soft_fail":false,"target_va":5375081504,"va":5371746589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39796561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e8513f5f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5371746314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3755782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e8064f3900","size":5,"soft_fail":false,"target_va":5375502352,"va":5371746565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10265740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e88ca49c00","size":5,"soft_fail":false,"target_va":5382011904,"va":5371746159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10265805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371746112,"raw_hex":"e8cda49c00","size":5,"soft_fail":false,"target_va":5382012016,"va":5371746206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":117,"record_offset":73684,"parent_record_offset":73264,"ancestor_record_offset":73264,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":73712,"parent_record_offset":73264,"ancestor_record_offset":73264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_date_time
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":117,"record_offset":73404,"parent_record_offset":73264,"ancestor_record_offset":73264,"inlinee":12244,"source_line":11,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":73888,"parent_record_offset":73864,"ancestor_record_offset":73264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":73864,"parent_record_offset":73808,"ancestor_record_offset":73264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":73836,"parent_record_offset":73808,"ancestor_record_offset":73264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":73808,"parent_record_offset":73784,"ancestor_record_offset":73264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":73784,"parent_record_offset":73760,"ancestor_record_offset":73264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":73520,"parent_record_offset":73484,"ancestor_record_offset":73264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":117,"record_offset":73484,"parent_record_offset":73404,"ancestor_record_offset":73264,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":117,"record_offset":73456,"parent_record_offset":73404,"ancestor_record_offset":73264,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FCD
// 首条位置: {"module":117,"inlinee":12237}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":72148,"parent_record_offset":72124,"ancestor_record_offset":72124,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":72196,"parent_record_offset":72172,"ancestor_record_offset":72124,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":117,"record_offset":72172,"parent_record_offset":72148,"ancestor_record_offset":72124,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00117.jsonl.gz"}
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
// 完整结构及原始类型记录: logging.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: logging.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: logging.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > (*)(bevy_ecs::event::reader::EventReader<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >)
// 候选参数类型: ["bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >"]
// [候选系统接口] system_input: bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >,ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >", "ref_mut$<bevy_ecs::world::World>"]
// [候选系统接口] system_input: bevy_ecs::system::input::In<enum2$<core::option::Option<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> > > >

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_date_time
// 信息 ID: 638c33082afd957502e8f777
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$1; module=117, inline_record=73404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_time
// 信息 ID: b14856e39d2d0c437a913114
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$0; module=117, inline_record=72388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging
// 信息 ID: f472d5fa7fe5ef3e1e152b08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd074 -> 0x2e4760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$0
// 信息 ID: d8992094436687a8d24198d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x2e2e77 -> 0x2e53f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$1
// 信息 ID: dd91e1aa2325da87ae6df0ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x2e2eb4 -> 0x2e5740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$2
// 信息 ID: 01b1d8277c085baa3ef57c82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging; module=117, inline_record=72124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
