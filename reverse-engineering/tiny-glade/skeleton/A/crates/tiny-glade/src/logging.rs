// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/logging.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3381d8fa7eb07d311d0841734795e4ad5bc42b8aa0cad3afe14a98a8da9277fd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: logging.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 186
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use fern; // namespace candidate
//   package identities: [{"name":"fern","source":"PDB_registry_path_only","version":"0.6.1"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"PDB_registry_path_only","version":"0.3.36"}]
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_date_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x308F
// 首条位置: {"module":186,"inlinee":12431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":11184,"parent_record_offset":11132,"ancestor_record_offset":11132,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":11240,"parent_record_offset":11132,"ancestor_record_offset":11132,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":186,"record_offset":11212,"parent_record_offset":11132,"ancestor_record_offset":11132,"inlinee":12428,"source_line":1271,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":11276,"parent_record_offset":11240,"ancestor_record_offset":11132,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x308B
// 首条位置: {"module":186,"inlinee":12427}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":10188,"parent_record_offset":10136,"ancestor_record_offset":10136,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":10244,"parent_record_offset":10136,"ancestor_record_offset":10136,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":186,"record_offset":10216,"parent_record_offset":10136,"ancestor_record_offset":10136,"inlinee":12428,"source_line":1271,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":10284,"parent_record_offset":10244,"ancestor_record_offset":10136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":7420,"rva":3072592,"va":5371781712}
// 编译实例 RVAs: 0x2ee250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t529909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8f5150800","size":5,"soft_fail":false,"target_va":5372314208,"va":5371784294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371310
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t534524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8fc270800","size":5,"soft_fail":false,"target_va":5372318480,"va":5371783951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t967800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e878c40e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371781891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t966614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8d6bf0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371783093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3388009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e869b23300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371783042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31194285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8adfcdb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371783854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5746083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8a3ad5700","size":5,"soft_fail":false,"target_va":5377529888,"va":5371783800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-476147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e80dbcf8ff","size":5,"soft_fail":false,"target_va":5371305968,"va":5371782110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31161952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8607edb01","size":5,"soft_fail":false,"target_va":5402945776,"va":5371783819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] colored::control::set_virtual_terminal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1357773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8cdb71400","size":5,"soft_fail":false,"target_va":5373139568,"va":5371781790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<fern::builders::OutputInner> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e87cf0ffff","size":5,"soft_fail":false,"target_va":5371780048,"va":5371784015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31196217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e83904dc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371783906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::builders::Dispatch::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1355528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e808af1400","size":5,"soft_fail":false,"target_va":5373139216,"va":5371783683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::builders::Dispatch::level_for<ref$<str$> >
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-1874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8aef8ffff","size":5,"soft_fail":false,"target_va":5371780848,"va":5371782717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4371006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e83eb24200","size":5,"soft_fail":false,"target_va":5376154000,"va":5371782989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5624954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e87ad45500","size":5,"soft_fail":false,"target_va":5377408016,"va":5371783057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::stdout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5631797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e835ef5500","size":5,"soft_fail":false,"target_va":5377413824,"va":5371782022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5656741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8a5505600","size":5,"soft_fail":false,"target_va":5377439776,"va":5371783030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5206650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e87a724f00","size":5,"soft_fail":false,"target_va":5376988416,"va":5371781761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::utc_offset::UtcOffset::local_offset_at
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5205554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371781712,"raw_hex":"e8326e4f00","size":5,"soft_fail":false,"target_va":5376987328,"va":5371781769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":9844,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":7592,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":9428,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::chain
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":186,"record_offset":7944,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12420,"source_line":198,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":186,"record_offset":7676,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12418,"source_line":171,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":186,"record_offset":7916,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12419,"source_line":224,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/builders.rs
//     evidence={"module":186,"record_offset":7648,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12417,"source_line":127,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/colors.rs
//     evidence={"module":186,"record_offset":7620,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12415,"source_line":158,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":186,"record_offset":8484,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12422,"source_line":1049,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":186,"record_offset":8400,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":186,"record_offset":8624,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":186,"record_offset":8540,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12423,"source_line":1028,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":186,"record_offset":8428,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":12421,"source_line":963,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":186,"record_offset":8596,"parent_record_offset":7420,"ancestor_record_offset":7420,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":7420,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":7420,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":7420,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":7756,"parent_record_offset":7724,"ancestor_record_offset":7420,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":7788,"parent_record_offset":7756,"ancestor_record_offset":7420,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":9040,"parent_record_offset":8984,"ancestor_record_offset":7420,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":7724,"parent_record_offset":7676,"ancestor_record_offset":7420,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":186,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":7420,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":186,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":7420,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":7420,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":8064,"parent_record_offset":8032,"ancestor_record_offset":7420,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":7420,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":8032,"parent_record_offset":7988,"ancestor_record_offset":7420,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":7988,"parent_record_offset":7944,"ancestor_record_offset":7420,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":7420,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":8824,"parent_record_offset":8624,"ancestor_record_offset":7420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7420,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":8096,"parent_record_offset":7988,"ancestor_record_offset":7420,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":186,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":7420,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":186,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":7420,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":186,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":7420,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":186,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":7420,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":186,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":7420,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":186,"record_offset":8656,"parent_record_offset":8624,"ancestor_record_offset":7420,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":186,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":7420,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":186,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":7420,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":186,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7420,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::logging::set_up_logging::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":186,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":7420,"inlinee":12425,"source_line":121,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":9992,"rva":3075744,"va":5371784864}
// 编译实例 RVAs: 0x2eeea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t528635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e8fb100800","size":5,"soft_fail":false,"target_va":5372314208,"va":5371785568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t964322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e8e2b60e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371785385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31194667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e82bfedb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371785456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1346074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e81a8a1400","size":5,"soft_fail":false,"target_va":5373131440,"va":5371785361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::formattable::sealed::Sealed::format
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2709363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e88da8d6ff","size":5,"soft_fail":false,"target_va":5369075648,"va":5371785006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5203498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e82a664f00","size":5,"soft_fail":false,"target_va":5376988416,"va":5371784913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5203499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371784864,"raw_hex":"e82b664f00","size":5,"soft_fail":false,"target_va":5376988464,"va":5371784960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":186,"record_offset":10496,"parent_record_offset":9992,"ancestor_record_offset":9992,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":10524,"parent_record_offset":9992,"ancestor_record_offset":9992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/colors.rs
//     evidence={"module":186,"record_offset":10420,"parent_record_offset":9992,"ancestor_record_offset":9992,"inlinee":12429,"source_line":230,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_time
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":186,"record_offset":10136,"parent_record_offset":9992,"ancestor_record_offset":9992,"inlinee":12427,"source_line":29,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":9992,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":10656,"parent_record_offset":10600,"ancestor_record_offset":9992,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":9992,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":9992,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":10188,"parent_record_offset":10136,"ancestor_record_offset":9992,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":10284,"parent_record_offset":10244,"ancestor_record_offset":9992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":10244,"parent_record_offset":10136,"ancestor_record_offset":9992,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] fern::colors::ColoredLevelConfig::get_color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fern-0.6.1/src/colors.rs
//     evidence={"module":186,"record_offset":10448,"parent_record_offset":10420,"ancestor_record_offset":9992,"inlinee":12430,"source_line":235,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":186,"record_offset":10216,"parent_record_offset":10136,"ancestor_record_offset":9992,"inlinee":12428,"source_line":1271,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":10992,"rva":3076608,"va":5371785728}
// 编译实例 RVAs: 0x2ef200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e80d0e0800","size":5,"soft_fail":false,"target_va":5372314208,"va":5371786318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t963500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e8acb30e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371786207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31194192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e850fcdb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371785931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1345252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e8e4861400","size":5,"soft_fail":false,"target_va":5373131440,"va":5371786183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::formattable::sealed::Sealed::format
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2710225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e82fa5d6ff","size":5,"soft_fail":false,"target_va":5369075648,"va":5371785868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5202636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e8cc624f00","size":5,"soft_fail":false,"target_va":5376988416,"va":5371785775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5202637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371785728,"raw_hex":"e8cd624f00","size":5,"soft_fail":false,"target_va":5376988464,"va":5371785822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":186,"record_offset":11440,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":11468,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_date_time
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":186,"record_offset":11132,"parent_record_offset":10992,"ancestor_record_offset":10992,"inlinee":12431,"source_line":11,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":10992,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":11600,"parent_record_offset":11544,"ancestor_record_offset":10992,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":11572,"parent_record_offset":11544,"ancestor_record_offset":10992,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":11544,"parent_record_offset":11516,"ancestor_record_offset":10992,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":11184,"parent_record_offset":11132,"ancestor_record_offset":10992,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":11276,"parent_record_offset":11240,"ancestor_record_offset":10992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":186,"record_offset":11240,"parent_record_offset":11132,"ancestor_record_offset":10992,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":186,"record_offset":11212,"parent_record_offset":11132,"ancestor_record_offset":10992,"inlinee":12428,"source_line":1271,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3089
// 首条位置: {"module":186,"inlinee":12425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":186,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":9872,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":186,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":9872,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":186,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":9872,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00186.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_date_time
// 信息 ID: 28eafe818c1480879e2c93bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$1; module=186, inline_record=11132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_time
// 信息 ID: 8f4eff84c051c62f005e9ae9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$0; module=186, inline_record=10136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging
// 信息 ID: be81c0522fb30d3d8033fd80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x399d9b -> 0x2ee250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$0
// 信息 ID: 989860b40c728542d1441ebd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::logging::set_up_logging::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x2edb87 -> 0x2eeea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$1
// 信息 ID: 6108960eb07bbe086f0219bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::logging::set_up_logging::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x2edbc4 -> 0x2ef200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$2
// 信息 ID: f518b6de311cb0a432aa0b2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging; module=186, inline_record=9872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
