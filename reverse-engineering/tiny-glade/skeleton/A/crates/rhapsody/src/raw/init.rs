// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/init.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 233fe66a54ac6fde82b7d388ff5b72ec1d4e5fff240f596de61d6e0e545ca98c, 35f6b5ac1d2c2da9a885c2be2350c35ac8ce6d35b778d4a05ac61fb92860ef61
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: init.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 6, 21, 73, 117, 126, 127, 170, 194, 197, 207, 222, 256, 559, 778, 779, 781, 1593, 1598, 1604, 1605, 1980, 3007, 3235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"PDB_registry_path_only","version":"1.1.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLoader::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":15124,"rva":7539552,"va":5376248672}
// 编译实例 RVAs: 0x730b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0900
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4590430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8a2f4b9ff","size":5,"soft_fail":false,"target_va":5371660544,"va":5376250969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3940101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8fbe0c3ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5376254304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3499096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8a89bcaff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376248787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3499375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8919acaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376249082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=5
//     target_aliases=28 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1078118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e89a8cefff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376249169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8a892ffff","size":5,"soft_fail":false,"target_va":5376225552,"va":5376253539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26724860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8fcc99701","size":5,"soft_fail":false,"target_va":5402978144,"va":5376253279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4944026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8668fb4ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5376249253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::entry::Entry::load_from
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10828290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8023aa500","size":5,"soft_fail":false,"target_va":5387078384,"va":5376250089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26726130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8f2ce9701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376253209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ash::entry::loaded::LoadingError> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8b8e6ffff","size":5,"soft_fail":false,"target_va":5376243264,"va":5376249731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-9653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e84bdaffff","size":5,"soft_fail":false,"target_va":5376244112,"va":5376253760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26726861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8cdd19701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376253262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] known_folders::win::get_known_folder_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10733098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e82ac6a300","size":5,"soft_fail":false,"target_va":5386982016,"va":5376248913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t1103726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e86ed71000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376250045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1150836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8748f1100","size":5,"soft_fail":false,"target_va":5377399792,"va":5376248951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1150411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8cb8d1100","size":5,"soft_fail":false,"target_va":5377399136,"va":5376248720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t1190716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e83c2b1200","size":5,"soft_fail":false,"target_va":5377439776,"va":5376249055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1191124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376248672,"raw_hex":"e8d42c1200","size":5,"soft_fail":false,"target_va":5377441024,"va":5376249895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*112(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376248672,"raw_hex":"ff5570","size":3,"soft_fail":false,"va":5376250638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15396,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":559,"record_offset":24164,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":16040,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":559,"record_offset":15544,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":17212,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":21528,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::try_enumerate_instance_version
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":559,"record_offset":20276,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":30661,"source_line":222,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_major
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":559,"record_offset":20432,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":30663,"source_line":41,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_minor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":559,"record_offset":20460,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":30664,"source_line":45,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::api_version_patch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":559,"record_offset":20488,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":30665,"source_line":49,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":20024,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":559,"record_offset":17576,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":21472,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":19588,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":559,"record_offset":18996,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":20892,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":16124,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":20404,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":17352,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":15352,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":18912,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":559,"record_offset":19616,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":559,"record_offset":19488,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":17180,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":17040,"parent_record_offset":15124,"ancestor_record_offset":15124,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":15124,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15124,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":15124,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15428,"parent_record_offset":15396,"ancestor_record_offset":15124,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16752,"parent_record_offset":16696,"ancestor_record_offset":15124,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":15124,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":17288,"parent_record_offset":17256,"ancestor_record_offset":15124,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":15776,"parent_record_offset":15748,"ancestor_record_offset":15124,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":15748,"parent_record_offset":15720,"ancestor_record_offset":15124,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":15924,"parent_record_offset":15748,"ancestor_record_offset":15124,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":15124,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":15684,"parent_record_offset":15656,"ancestor_record_offset":15124,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":15124,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":15656,"parent_record_offset":15628,"ancestor_record_offset":15124,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":15124,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":15628,"parent_record_offset":15600,"ancestor_record_offset":15124,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":15572,"parent_record_offset":15544,"ancestor_record_offset":15124,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":559,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":15124,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":15124,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":20756,"parent_record_offset":20732,"ancestor_record_offset":15124,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":17256,"parent_record_offset":17212,"ancestor_record_offset":15124,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":15124,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":15720,"parent_record_offset":15684,"ancestor_record_offset":15124,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":15124,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":559,"record_offset":20316,"parent_record_offset":20276,"ancestor_record_offset":15124,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":559,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":15124,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":559,"record_offset":20364,"parent_record_offset":20340,"ancestor_record_offset":15124,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":559,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":15124,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":559,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":15124,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":15124,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":20916,"parent_record_offset":20892,"ancestor_record_offset":15124,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":16524,"parent_record_offset":16168,"ancestor_record_offset":15124,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":15960,"parent_record_offset":15684,"ancestor_record_offset":15124,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":16268,"parent_record_offset":16232,"ancestor_record_offset":15124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":15124,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":15124,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":19060,"parent_record_offset":19024,"ancestor_record_offset":15124,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":15124,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":17320,"parent_record_offset":17212,"ancestor_record_offset":15124,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":16232,"parent_record_offset":16168,"ancestor_record_offset":15124,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":21560,"parent_record_offset":21528,"ancestor_record_offset":15124,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":21592,"parent_record_offset":21560,"ancestor_record_offset":15124,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":18936,"parent_record_offset":18912,"ancestor_record_offset":15124,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":19088,"parent_record_offset":19024,"ancestor_record_offset":15124,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":19024,"parent_record_offset":18996,"ancestor_record_offset":15124,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":20792,"parent_record_offset":20756,"ancestor_record_offset":15124,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":19528,"parent_record_offset":19488,"ancestor_record_offset":15124,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":15124,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":20816,"parent_record_offset":20792,"ancestor_record_offset":15124,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":20848,"parent_record_offset":20756,"ancestor_record_offset":15124,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":16168,"parent_record_offset":16124,"ancestor_record_offset":15124,"inlinee":30658,"source_line":211,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":15124,"inlinee":30660,"source_line":214,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":559,"record_offset":16204,"parent_record_offset":16168,"ancestor_record_offset":15124,"inlinee":16002,"source_line":229,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":559,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":15124,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":559,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":15124,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":559,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":15124,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":15124,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":15124,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$10::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x296A
// 首条位置: {"module":126,"record_offset":8952,"rva":2376080,"va":5371085200}
// 编译实例 RVAs: 0x244190, 0x3c4ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6498563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085200,"raw_hex":"e803296300","size":5,"soft_fail":false,"target_va":5377584016,"va":5371085448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6498620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085200,"raw_hex":"e83c296300","size":5,"soft_fail":false,"target_va":5377584208,"va":5371085583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6500574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085200,"raw_hex":"e8de306300","size":5,"soft_fail":false,"target_va":5377585888,"va":5371085309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6498650","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371085200,"raw_hex":"e95a296300","size":5,"soft_fail":false,"target_va":5377583920,"va":5371085265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$14::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":33412,"rva":7549536,"va":5376258656}
// 编译实例 RVAs: 0x733260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayer::vendor
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":9704,"rva":7537328,"va":5376246448}
// 编译实例 RVAs: 0x7302b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1342902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246448,"raw_hex":"e8b67d1400","size":5,"soft_fail":false,"target_va":5377589536,"va":5376246629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26733917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246448,"raw_hex":"e85ded9701","size":5,"soft_fail":false,"target_va":5402980656,"va":5376246734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":10076,"parent_record_offset":9704,"ancestor_record_offset":9704,"inlinee":19447,"source_line":1264,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":9848,"parent_record_offset":9704,"ancestor_record_offset":9704,"inlinee":7369,"source_line":2217,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":10368,"parent_record_offset":9704,"ancestor_record_offset":9704,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":9964,"parent_record_offset":9932,"ancestor_record_offset":9704,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":9992,"parent_record_offset":9964,"ancestor_record_offset":9704,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":9704,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":10136,"parent_record_offset":10100,"ancestor_record_offset":9704,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":9704,"inlinee":5503,"source_line":2554,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":9704,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":10168,"parent_record_offset":10136,"ancestor_record_offset":9704,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":559,"record_offset":10200,"parent_record_offset":10100,"ancestor_record_offset":9704,"inlinee":5066,"source_line":27,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":559,"record_offset":10232,"parent_record_offset":10200,"ancestor_record_offset":9704,"inlinee":5067,"source_line":38,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":9704,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9704,"inlinee":16677,"source_line":418,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":9704,"inlinee":7373,"source_line":962,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":9876,"parent_record_offset":9848,"ancestor_record_offset":9704,"inlinee":7372,"source_line":994,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":9704,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":9704,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 87, 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2822
// 首条位置: {"module":117,"inlinee":10274}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":117,"record_offset":99788,"parent_record_offset":99764,"ancestor_record_offset":99764,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 87, 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27E8
// 首条位置: {"module":117,"inlinee":10216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":117,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":6400,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":117,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":6400,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":6400,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":6400,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 87, 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27E6
// 首条位置: {"module":117,"inlinee":10214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":117,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":6248,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":6332,"parent_record_offset":6284,"ancestor_record_offset":6248,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":6248,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":6248,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":117,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":6248,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":117,"record_offset":6400,"parent_record_offset":6368,"ancestor_record_offset":6248,"inlinee":10216,"source_line":87,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":6368,"parent_record_offset":6284,"ancestor_record_offset":6248,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2850
// 首条位置: {"module":117,"inlinee":10320}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":117,"record_offset":86652,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":86804,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":86560,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":86772,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":85396,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":86596,"parent_record_offset":85352,"ancestor_record_offset":85352,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":87184,"parent_record_offset":87160,"ancestor_record_offset":85352,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":87160,"parent_record_offset":87136,"ancestor_record_offset":85352,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":87016,"parent_record_offset":86988,"ancestor_record_offset":85352,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":117,"record_offset":86988,"parent_record_offset":86960,"ancestor_record_offset":85352,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":117,"record_offset":86828,"parent_record_offset":86804,"ancestor_record_offset":85352,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":117,"record_offset":86296,"parent_record_offset":86272,"ancestor_record_offset":85352,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":85596,"parent_record_offset":85432,"ancestor_record_offset":85352,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":85636,"parent_record_offset":85596,"ancestor_record_offset":85352,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":85772,"parent_record_offset":85432,"ancestor_record_offset":85352,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":117,"record_offset":87324,"parent_record_offset":87296,"ancestor_record_offset":85352,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":86852,"parent_record_offset":86828,"ancestor_record_offset":85352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":85556,"parent_record_offset":85432,"ancestor_record_offset":85352,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":86320,"parent_record_offset":86296,"ancestor_record_offset":85352,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":117,"record_offset":86344,"parent_record_offset":86320,"ancestor_record_offset":85352,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":117,"record_offset":86272,"parent_record_offset":86244,"ancestor_record_offset":85352,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":117,"record_offset":86244,"parent_record_offset":86216,"ancestor_record_offset":85352,"inlinee":10216,"source_line":87,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":117,"record_offset":85740,"parent_record_offset":85716,"ancestor_record_offset":85352,"inlinee":10214,"source_line":87,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":117,"record_offset":85676,"parent_record_offset":85432,"ancestor_record_offset":85352,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":86136,"parent_record_offset":86104,"ancestor_record_offset":85352,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":117,"record_offset":85432,"parent_record_offset":85396,"ancestor_record_offset":85352,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":86216,"parent_record_offset":86136,"ancestor_record_offset":85352,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":85716,"parent_record_offset":85432,"ancestor_record_offset":85352,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":8480,"rva":3433424,"va":5372142544}
// 编译实例 RVAs: 0x3463d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14000c910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3382191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142544,"raw_hex":"e85164ccff","size":5,"soft_fail":false,"target_va":5368760592,"va":5372142778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14000dad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3377588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142544,"raw_hex":"e84c76ccff","size":5,"soft_fail":false,"target_va":5368765136,"va":5372142719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400105d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3366704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142544,"raw_hex":"e8d0a0ccff","size":5,"soft_fail":false,"target_va":5368776144,"va":5372142843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140010ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3364139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142544,"raw_hex":"e8d5aaccff","size":5,"soft_fail":false,"target_va":5368778480,"va":5372142614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":8848,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":222,"record_offset":8684,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":222,"record_offset":8792,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":222,"record_offset":8900,"parent_record_offset":8872,"ancestor_record_offset":8480,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":222,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":8480,"inlinee":4979,"source_line":1098,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2962
// 首条位置: {"module":126,"inlinee":10594}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77D9
// 首条位置: {"module":559,"record_offset":10548,"rva":7537632,"va":5376246752}
// 编译实例 RVAs: 0x7303e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27346505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246752,"raw_hex":"e84946a101","size":5,"soft_fail":false,"target_va":5403593344,"va":5376246834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":10700,"parent_record_offset":10548,"ancestor_record_offset":10548,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":10548,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":10548,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":10772,"parent_record_offset":10732,"ancestor_record_offset":10548,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10548,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10548,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":10732,"parent_record_offset":10700,"ancestor_record_offset":10548,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":10884,"parent_record_offset":10856,"ancestor_record_offset":10548,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":10804,"parent_record_offset":10700,"ancestor_record_offset":10548,"inlinee":30646,"source_line":95,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$1::contains_layer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77B6
// 首条位置: {"module":559,"inlinee":30646}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":10804,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10804,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10804,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10804,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":10884,"parent_record_offset":10856,"ancestor_record_offset":10804,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77DA
// 首条位置: {"module":559,"record_offset":11032,"rva":7537744,"va":5376246864}
// 编译实例 RVAs: 0x730450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27346393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246864,"raw_hex":"e8d945a101","size":5,"soft_fail":false,"target_va":5403593344,"va":5376246946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":11184,"parent_record_offset":11032,"ancestor_record_offset":11032,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":11032,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":11316,"parent_record_offset":11288,"ancestor_record_offset":11032,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":11256,"parent_record_offset":11216,"ancestor_record_offset":11032,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":11032,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":11032,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":11216,"parent_record_offset":11184,"ancestor_record_offset":11032,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":11368,"parent_record_offset":11340,"ancestor_record_offset":11032,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":11288,"parent_record_offset":11184,"ancestor_record_offset":11032,"inlinee":30648,"source_line":105,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$1::contains_vendor::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77B8
// 首条位置: {"module":559,"inlinee":30648}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":11316,"parent_record_offset":11288,"ancestor_record_offset":11288,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":11288,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":11396,"parent_record_offset":11368,"ancestor_record_offset":11288,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":11288,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":11288,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":11368,"parent_record_offset":11340,"ancestor_record_offset":11288,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x283C
// 首条位置: {"module":117,"inlinee":10300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":62600,"parent_record_offset":62484,"ancestor_record_offset":62484,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":117,"record_offset":62536,"parent_record_offset":62484,"ancestor_record_offset":62484,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":62508,"parent_record_offset":62484,"ancestor_record_offset":62484,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":117,"record_offset":62572,"parent_record_offset":62484,"ancestor_record_offset":62484,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13AA
// 首条位置: {"module":6,"inlinee":5034}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$19::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2158
// 首条位置: {"module":73,"inlinee":8536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":73,"record_offset":17072,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":73,"record_offset":17096,"parent_record_offset":17072,"ancestor_record_offset":17044,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":73,"record_offset":17116,"parent_record_offset":17096,"ancestor_record_offset":17044,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":73,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":17044,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$21::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3126
// 首条位置: {"module":194,"inlinee":12582}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$71::partial_cmp
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":194,"record_offset":29360,"parent_record_offset":29324,"ancestor_record_offset":29324,"inlinee":12569,"source_line":362,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":29536,"parent_record_offset":29324,"ancestor_record_offset":29324,"inlinee":12584,"source_line":1549,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::partial_cmp
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":29384,"parent_record_offset":29360,"ancestor_record_offset":29324,"inlinee":12571,"source_line":3181,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$67::cmp
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":29488,"parent_record_offset":29456,"ancestor_record_offset":29324,"inlinee":7055,"source_line":1565,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$3::partial_cmp
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":29408,"parent_record_offset":29384,"ancestor_record_offset":29324,"inlinee":12573,"source_line":38,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$7::partial_compare
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":29432,"parent_record_offset":29408,"ancestor_record_offset":29324,"inlinee":12575,"source_line":137,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$9::compare
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":29456,"parent_record_offset":29432,"ancestor_record_offset":29324,"inlinee":7053,"source_line":191,"archive":"_pdb/A/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$23::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2176
// 首条位置: {"module":73,"inlinee":8566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstance::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":11516,"rva":7537856,"va":5376246976}
// 编译实例 RVAs: 0x7304c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4686632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e8d87cb8ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5376248403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0900
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4586695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e83903baff","size":5,"soft_fail":false,"target_va":5371660544,"va":5376247234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3497410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e83ea2caff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376247101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3498127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e8719fcaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376247834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141182080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10819946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e86a19a500","size":5,"soft_fail":false,"target_va":5387067520,"va":5376247569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27345800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e88843a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376247555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27347268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e84449a101","size":5,"soft_fail":false,"target_va":5403594400,"va":5376247127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26729892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e8a4dd9701","size":5,"soft_fail":false,"target_va":5402978144,"va":5376248247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1278890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e8aa831300","size":5,"soft_fail":false,"target_va":5377527120,"va":5376248225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10819715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e88318a500","size":5,"soft_fail":false,"target_va":5387067104,"va":5376247384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26731912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e888e59701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376248211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1106417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e8f1e11000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376247354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::instance::InstanceBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10820251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376246976,"raw_hex":"e89b1aa500","size":5,"soft_fail":false,"target_va":5387068304,"va":5376248048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":12660,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":12964,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":13444,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":13184,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":11716,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":12720,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":11688,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":12692,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":559,"record_offset":12992,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":559,"record_offset":12564,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":13020,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":30652,"source_line":360,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::InstanceBuilder::debug_markers
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":559,"record_offset":14256,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":30656,"source_line":40,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":11516,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11516,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":11516,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":12012,"parent_record_offset":11920,"ancestor_record_offset":11516,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":13656,"parent_record_offset":13600,"ancestor_record_offset":11516,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":13628,"parent_record_offset":13600,"ancestor_record_offset":11516,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":11920,"parent_record_offset":11892,"ancestor_record_offset":11516,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":11516,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":12132,"parent_record_offset":11892,"ancestor_record_offset":11516,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":13600,"parent_record_offset":13572,"ancestor_record_offset":11516,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":12824,"parent_record_offset":12796,"ancestor_record_offset":11516,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":11516,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11836,"parent_record_offset":11800,"ancestor_record_offset":11516,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11864,"parent_record_offset":11836,"ancestor_record_offset":11516,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":11744,"parent_record_offset":11716,"ancestor_record_offset":11516,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":13124,"parent_record_offset":13020,"ancestor_record_offset":11516,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11516,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":13984,"parent_record_offset":13960,"ancestor_record_offset":11516,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":11516,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":11516,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":559,"record_offset":12176,"parent_record_offset":11800,"ancestor_record_offset":11516,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":559,"record_offset":11772,"parent_record_offset":11744,"ancestor_record_offset":11516,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":559,"record_offset":11800,"parent_record_offset":11772,"ancestor_record_offset":11516,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] ash::entry::Entry::enumerate_instance_layer_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":559,"record_offset":13056,"parent_record_offset":13020,"ancestor_record_offset":11516,"inlinee":30654,"source_line":269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":11956,"parent_record_offset":11920,"ancestor_record_offset":11516,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":11516,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":11516,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12376,"parent_record_offset":12348,"ancestor_record_offset":11516,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":11516,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":11516,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":12300,"parent_record_offset":12272,"ancestor_record_offset":11516,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":11516,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":12440,"parent_record_offset":12300,"ancestor_record_offset":11516,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":14136,"parent_record_offset":14108,"ancestor_record_offset":11516,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":13300,"parent_record_offset":13184,"ancestor_record_offset":11516,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":11516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":11516,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":13268,"parent_record_offset":13208,"ancestor_record_offset":11516,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":13240,"parent_record_offset":13208,"ancestor_record_offset":11516,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":12088,"parent_record_offset":12060,"ancestor_record_offset":11516,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":12488,"parent_record_offset":12464,"ancestor_record_offset":11516,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":13084,"parent_record_offset":13020,"ancestor_record_offset":11516,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":11516,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12860,"parent_record_offset":12824,"ancestor_record_offset":11516,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12592,"parent_record_offset":12564,"ancestor_record_offset":11516,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12620,"parent_record_offset":12592,"ancestor_record_offset":11516,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":11516,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":12916,"parent_record_offset":12824,"ancestor_record_offset":11516,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$3::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":11516,"inlinee":30650,"source_line":146,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$3::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77BA
// 首条位置: {"module":559,"inlinee":30650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":12324,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12376,"parent_record_offset":12348,"ancestor_record_offset":12324,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":559,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":12324,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77C2
// 首条位置: {"module":559,"inlinee":30658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":16524,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":16232,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":559,"record_offset":16204,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":16002,"source_line":229,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":16168,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16752,"parent_record_offset":16696,"ancestor_record_offset":16168,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":16168,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":16168,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":16268,"parent_record_offset":16232,"ancestor_record_offset":16168,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$4::new::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":16168,"inlinee":30660,"source_line":214,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":16168,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77C4
// 首条位置: {"module":559,"inlinee":30660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":16624,"parent_record_offset":16560,"ancestor_record_offset":16560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":559,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":16560,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":16560,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16752,"parent_record_offset":16696,"ancestor_record_offset":16560,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16724,"parent_record_offset":16696,"ancestor_record_offset":16560,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":16696,"parent_record_offset":16672,"ancestor_record_offset":16560,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":16648,"parent_record_offset":16624,"ancestor_record_offset":16560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 360
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77BC
// 首条位置: {"module":559,"inlinee":30652}
// 编译实例 RVAs: 0x732200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10812622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254464,"raw_hex":"e8cefca400","size":5,"soft_fail":false,"target_va":5387067104,"va":5376254477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26725529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254464,"raw_hex":"e899cc9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376254594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":13124,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::enumerate_instance_layer_properties
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":559,"record_offset":13056,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":30654,"source_line":269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":25848,"parent_record_offset":25576,"ancestor_record_offset":25576,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":13084,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":25872,"parent_record_offset":25848,"ancestor_record_offset":25576,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":13020,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$4::implicit_layers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17D8
// 首条位置: {"module":21,"inlinee":6104}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":21,"record_offset":11780,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":5874,"source_line":323,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":12952,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":21,"record_offset":12352,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":6105,"source_line":1108,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11728,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":11728,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":13104,"parent_record_offset":13080,"ancestor_record_offset":11728,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":12164,"parent_record_offset":12128,"ancestor_record_offset":11728,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":13080,"parent_record_offset":13024,"ancestor_record_offset":11728,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":13052,"parent_record_offset":13024,"ancestor_record_offset":11728,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":11728,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":12100,"parent_record_offset":12072,"ancestor_record_offset":11728,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":12284,"parent_record_offset":12100,"ancestor_record_offset":11728,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":11728,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":11980,"parent_record_offset":11948,"ancestor_record_offset":11728,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":11728,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":11884,"parent_record_offset":11852,"ancestor_record_offset":11728,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":11916,"parent_record_offset":11884,"ancestor_record_offset":11728,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":11852,"parent_record_offset":11820,"ancestor_record_offset":11728,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":21,"record_offset":11820,"parent_record_offset":11780,"ancestor_record_offset":11728,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":21,"record_offset":12072,"parent_record_offset":11980,"ancestor_record_offset":11728,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":21,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11728,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":21,"record_offset":12016,"parent_record_offset":11980,"ancestor_record_offset":11728,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":12976,"parent_record_offset":12952,"ancestor_record_offset":11728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":12240,"parent_record_offset":12212,"ancestor_record_offset":11728,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsInstanceLoader::supports_modern_layer_filtering
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77DD
// 首条位置: {"module":1605,"inlinee":30685}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$25::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":12424,"rva":3435504,"va":5372144624}
// 编译实例 RVAs: 0x346bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5439257","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372144624,"raw_hex":"e919ff5200","size":5,"soft_fail":false,"target_va":5377583920,"va":5372144658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDeviceInitState::is_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 410
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":25908,"rva":5545952,"va":5374255072}
// 编译实例 RVAs: 0x549fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$27::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":8100,"rva":3432784,"va":5372141904}
// 编译实例 RVAs: 0x346150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5441977","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372141904,"raw_hex":"e9b9095300","size":5,"soft_fail":false,"target_va":5377583920,"va":5372141938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$29::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18D0D
// 首条位置: {"module":3235,"inlinee":101645}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$31::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35E3
// 首条位置: {"module":256,"inlinee":13795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":35500,"parent_record_offset":35096,"ancestor_record_offset":35096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":256,"record_offset":35164,"parent_record_offset":35096,"ancestor_record_offset":35096,"inlinee":13797,"source_line":42,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":35440,"parent_record_offset":35412,"ancestor_record_offset":35096,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":35412,"parent_record_offset":35384,"ancestor_record_offset":35096,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":35608,"parent_record_offset":35576,"ancestor_record_offset":35096,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":35384,"parent_record_offset":35360,"ancestor_record_offset":35096,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":35212,"parent_record_offset":35164,"ancestor_record_offset":35096,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":35288,"parent_record_offset":35164,"ancestor_record_offset":35096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$32::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x335F
// 首条位置: {"module":222,"inlinee":13151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_supported
// 记录形式: compiled_procedure; instances=1
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":26048,"rva":7545488,"va":5376254608}
// 编译实例 RVAs: 0x732290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254608,"raw_hex":"e822d7ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376254665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254608,"raw_hex":"e829010000","size":5,"soft_fail":false,"target_va":5376254928,"va":5376254626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254608,"raw_hex":"e85a040000","size":5,"soft_fail":false,"target_va":5376255888,"va":5376254769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26192,"parent_record_offset":26048,"ancestor_record_offset":26048,"inlinee":30667,"source_line":476,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":26220,"parent_record_offset":26192,"ancestor_record_offset":26048,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26276,"parent_record_offset":26192,"ancestor_record_offset":26048,"inlinee":30669,"source_line":604,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26248,"parent_record_offset":26192,"ancestor_record_offset":26048,"inlinee":30668,"source_line":581,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 476
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77CB
// 首条位置: {"module":559,"inlinee":30667}
// 编译实例 RVAs: 0x732340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254784,"raw_hex":"e872d6ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376254841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254784,"raw_hex":"e879000000","size":5,"soft_fail":false,"target_va":5376254928,"va":5376254802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":559,"record_offset":26220,"parent_record_offset":26192,"ancestor_record_offset":26192,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26276,"parent_record_offset":26192,"ancestor_record_offset":26192,"inlinee":30669,"source_line":604,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26248,"parent_record_offset":26192,"ancestor_record_offset":26192,"inlinee":30668,"source_line":581,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 记录形式: compiled_procedure; instances=1
// 源码行: 480
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":26556,"rva":7545808,"va":5376254928}
// 编译实例 RVAs: 0x7323d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254928,"raw_hex":"e8f1d5ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376254970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376254928,"raw_hex":"e8bb010000","size":5,"soft_fail":false,"target_va":5376255888,"va":5376255440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$17::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":559,"record_offset":26728,"parent_record_offset":26556,"ancestor_record_offset":26556,"inlinee":30671,"source_line":371,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":26704,"parent_record_offset":26556,"ancestor_record_offset":26556,"inlinee":30668,"source_line":581,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":26776,"parent_record_offset":26752,"ancestor_record_offset":26556,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":26752,"parent_record_offset":26728,"ancestor_record_offset":26556,"inlinee":5479,"source_line":1672,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":26936,"parent_record_offset":26908,"ancestor_record_offset":26556,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":27032,"parent_record_offset":26848,"ancestor_record_offset":26556,"inlinee":12580,"source_line":1565,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":559,"record_offset":26964,"parent_record_offset":26908,"ancestor_record_offset":26556,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":559,"record_offset":26908,"parent_record_offset":26880,"ancestor_record_offset":26556,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":559,"record_offset":26880,"parent_record_offset":26848,"ancestor_record_offset":26556,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":26556,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$3::partial_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":26800,"parent_record_offset":26776,"ancestor_record_offset":26556,"inlinee":12573,"source_line":38,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$7::partial_compare
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":26824,"parent_record_offset":26800,"ancestor_record_offset":26556,"inlinee":12575,"source_line":137,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$8::compare
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":26556,"inlinee":16459,"source_line":168,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=34
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77CC
// 首条位置: {"module":559,"inlinee":30668}
// 编译实例 RVAs: 0x732730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376255792,"raw_hex":"e89ad2ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376255825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11665","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376255792,"raw_hex":"e96fd2ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376255868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
// 记录形式: compiled_procedure; instances=1
// 源码行: 585
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":29056,"rva":7546768,"va":5376255888}
// 编译实例 RVAs: 0x732790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f9f0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376255888,"raw_hex":"e830d2ffff","size":5,"soft_fail":false,"target_va":5376244208,"va":5376255931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 604
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77CD
// 首条位置: {"module":559,"inlinee":30669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":9036,"rva":3434144,"va":5372143264}
// 编译实例 RVAs: 0x3466a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ad60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2931659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e83544d3ff","size":5,"soft_fail":false,"target_va":5369212256,"va":5372143910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e0eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2513446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8daa5d9ff","size":5,"soft_fail":false,"target_va":5369630384,"va":5372143825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2494865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e86feed9ff","size":5,"soft_fail":false,"target_va":5369649008,"va":5372143868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d06f0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-484197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e89b9cf8ff","size":5,"soft_fail":false,"target_va":5371660016,"va":5372144208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t107684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8a4a40100","size":5,"soft_fail":false,"target_va":5372251872,"va":5372144183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t245837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e84dc00300","size":5,"soft_fail":false,"target_va":5372390432,"va":5372144590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140397ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t333998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8ae180500","size":5,"soft_fail":false,"target_va":5372477424,"va":5372143421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t606202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8fa3f0900","size":5,"soft_fail":false,"target_va":5372749696,"va":5372143489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t605679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8ef3d0900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372144028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140729ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4076058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e81a323e00","size":5,"soft_fail":false,"target_va":5376219840,"va":5372143777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30833828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e8a47cd601","size":5,"soft_fail":false,"target_va":5402978144,"va":5372144311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5210372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e804814f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5372143399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::enumerate_physical_devices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4071691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372143264,"raw_hex":"e80b213e00","size":5,"soft_fail":false,"target_va":5376215344,"va":5372143648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":222,"record_offset":11344,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":222,"record_offset":9632,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":222,"record_offset":9540,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":10312,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":10140,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":222,"record_offset":9456,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":222,"record_offset":9484,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":222,"record_offset":10592,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":222,"record_offset":10228,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":10648,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":222,"record_offset":9568,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":222,"record_offset":9604,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":222,"record_offset":10056,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":222,"record_offset":10452,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":222,"record_offset":9512,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":222,"record_offset":9372,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9036,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":9036,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":9036,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":9036,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":9720,"parent_record_offset":9688,"ancestor_record_offset":9036,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":10780,"parent_record_offset":10724,"ancestor_record_offset":9036,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":222,"record_offset":9660,"parent_record_offset":9632,"ancestor_record_offset":9036,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":10752,"parent_record_offset":10724,"ancestor_record_offset":9036,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":11576,"parent_record_offset":11548,"ancestor_record_offset":9036,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":11548,"parent_record_offset":11520,"ancestor_record_offset":9036,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":11724,"parent_record_offset":11548,"ancestor_record_offset":9036,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":9036,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":11484,"parent_record_offset":11456,"ancestor_record_offset":9036,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":11456,"parent_record_offset":11428,"ancestor_record_offset":9036,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":11400,"parent_record_offset":11372,"ancestor_record_offset":9036,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":11428,"parent_record_offset":11400,"ancestor_record_offset":9036,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":11372,"parent_record_offset":11344,"ancestor_record_offset":9036,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":222,"record_offset":11900,"parent_record_offset":11864,"ancestor_record_offset":9036,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":222,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":9036,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":11520,"parent_record_offset":11484,"ancestor_record_offset":9036,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":10252,"parent_record_offset":10228,"ancestor_record_offset":9036,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":9036,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":222,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":9036,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":222,"record_offset":10276,"parent_record_offset":10252,"ancestor_record_offset":9036,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":222,"record_offset":10168,"parent_record_offset":10140,"ancestor_record_offset":9036,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":222,"record_offset":11240,"parent_record_offset":11212,"ancestor_record_offset":9036,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":222,"record_offset":11760,"parent_record_offset":11484,"ancestor_record_offset":9036,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":9036,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":222,"record_offset":10196,"parent_record_offset":10140,"ancestor_record_offset":9036,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":222,"record_offset":11932,"parent_record_offset":11900,"ancestor_record_offset":9036,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":9796,"parent_record_offset":9768,"ancestor_record_offset":9036,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":222,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":9036,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":222,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":9036,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":222,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":9036,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":222,"record_offset":11960,"parent_record_offset":11900,"ancestor_record_offset":9036,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":222,"record_offset":11992,"parent_record_offset":11864,"ancestor_record_offset":9036,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00222.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::new::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 657
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17F6
// 首条位置: {"module":21,"inlinee":6134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":19316,"parent_record_offset":19256,"ancestor_record_offset":19256,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":21,"record_offset":19288,"parent_record_offset":19256,"ancestor_record_offset":19256,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":19400,"parent_record_offset":19256,"ancestor_record_offset":19256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":19556,"parent_record_offset":19532,"ancestor_record_offset":19256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":19532,"parent_record_offset":19476,"ancestor_record_offset":19256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":19256,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":19504,"parent_record_offset":19476,"ancestor_record_offset":19256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":19476,"parent_record_offset":19448,"ancestor_record_offset":19256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":19340,"parent_record_offset":19316,"ancestor_record_offset":19256,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":19256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 668
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":29192,"rva":7547136,"va":5376256256}
// 编译实例 RVAs: 0x732900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402db2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4553729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8ff83baff","size":5,"soft_fail":false,"target_va":5371704048,"va":5376257772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3507423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8217bcaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376257130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406da180
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-363816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8d872faff","size":5,"soft_fail":false,"target_va":5375893888,"va":5376257699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8e8b3ffff","size":5,"soft_fail":false,"target_va":5376238416,"va":5376257891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27336243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8331ea101","size":5,"soft_fail":false,"target_va":5403593344,"va":5376257096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27336701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8fd1fa101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376256654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4947869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e86380b4ff","size":5,"soft_fail":false,"target_va":5371309360,"va":5376257224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1106898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8d2e31000","size":5,"soft_fail":false,"target_va":5377364336,"va":5376257433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e890c6ffff","size":5,"soft_fail":false,"target_va":5376243104,"va":5376257803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e8cbc8ffff","size":5,"soft_fail":false,"target_va":5376243392,"va":5376257520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] dxgi_enumerate_gpus::enumerate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10823212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e82c26a500","size":5,"soft_fail":false,"target_va":5387079568,"va":5376256351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1097273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376256256,"raw_hex":"e839be1000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376256498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":29380,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":29632,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":29576,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":29740,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":29492,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":29520,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":29944,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":30320,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":30000,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":559,"record_offset":29548,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":559,"record_offset":29408,"parent_record_offset":29192,"ancestor_record_offset":29192,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":30472,"parent_record_offset":30448,"ancestor_record_offset":29192,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":30448,"parent_record_offset":30392,"ancestor_record_offset":29192,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":30420,"parent_record_offset":30392,"ancestor_record_offset":29192,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":30392,"parent_record_offset":30368,"ancestor_record_offset":29192,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":30180,"parent_record_offset":30096,"ancestor_record_offset":29192,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":30776,"parent_record_offset":30752,"ancestor_record_offset":29192,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":31112,"parent_record_offset":31088,"ancestor_record_offset":29192,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":559,"record_offset":30208,"parent_record_offset":30180,"ancestor_record_offset":29192,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":31216,"parent_record_offset":31188,"ancestor_record_offset":29192,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":30124,"parent_record_offset":30096,"ancestor_record_offset":29192,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":30148,"parent_record_offset":30124,"ancestor_record_offset":29192,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":30344,"parent_record_offset":30320,"ancestor_record_offset":29192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":29600,"parent_record_offset":29576,"ancestor_record_offset":29192,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":29800,"parent_record_offset":29740,"ancestor_record_offset":29192,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":29772,"parent_record_offset":29740,"ancestor_record_offset":29192,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":29708,"parent_record_offset":29632,"ancestor_record_offset":29192,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":30236,"parent_record_offset":30208,"ancestor_record_offset":29192,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":30264,"parent_record_offset":30236,"ancestor_record_offset":29192,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":30028,"parent_record_offset":30000,"ancestor_record_offset":29192,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":29432,"parent_record_offset":29408,"ancestor_record_offset":29192,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":29456,"parent_record_offset":29432,"ancestor_record_offset":29192,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$7::missing_devices::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":30096,"parent_record_offset":30000,"ancestor_record_offset":29192,"inlinee":30673,"source_line":689,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::missing_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 689
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77D1
// 首条位置: {"module":559,"inlinee":30673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":30180,"parent_record_offset":30096,"ancestor_record_offset":30096,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":30124,"parent_record_offset":30096,"ancestor_record_offset":30096,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":559,"record_offset":30208,"parent_record_offset":30180,"ancestor_record_offset":30096,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":30148,"parent_record_offset":30124,"ancestor_record_offset":30096,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":30236,"parent_record_offset":30208,"ancestor_record_offset":30096,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":30264,"parent_record_offset":30236,"ancestor_record_offset":30096,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::available_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 711
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":31596,"rva":7548800,"va":5376257920}
// 编译实例 RVAs: 0x732f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10782657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257920,"raw_hex":"e8c187a400","size":5,"soft_fail":false,"target_va":5387040608,"va":5376257946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":31836,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":31752,"parent_record_offset":31596,"ancestor_record_offset":31596,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":31596,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":31800,"parent_record_offset":31776,"ancestor_record_offset":31596,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":31776,"parent_record_offset":31752,"ancestor_record_offset":31596,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::available_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1817E
// 首条位置: {"module":3007,"inlinee":98686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":1328,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":2504,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":1680,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":30417,"source_line":130,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":2196,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":30419,"source_line":162,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":1356,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":13797,"source_line":42,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3007,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":1260,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":1712,"parent_record_offset":1680,"ancestor_record_offset":1260,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":3007,"record_offset":1740,"parent_record_offset":1712,"ancestor_record_offset":1260,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3007,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3007,"record_offset":2040,"parent_record_offset":1948,"ancestor_record_offset":1260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":3007,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1260,"inlinee":30390,"source_line":49,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":3007,"record_offset":2136,"parent_record_offset":2040,"ancestor_record_offset":1260,"inlinee":30396,"source_line":53,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":1876,"parent_record_offset":1680,"ancestor_record_offset":1260,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3007,"record_offset":1404,"parent_record_offset":1356,"ancestor_record_offset":1260,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":1480,"parent_record_offset":1356,"ancestor_record_offset":1260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3007,"record_offset":2012,"parent_record_offset":1948,"ancestor_record_offset":1260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":1260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":1260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":1260,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3007,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1260,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":1260,"inlinee":30392,"source_line":134,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":1260,"inlinee":30400,"source_line":166,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::create_device
// 记录形式: compiled_procedure; instances=1
// 源码行: 733
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":31896,"rva":7548848,"va":5376257968}
// 编译实例 RVAs: 0x732fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3508574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257968,"raw_hex":"e8a276caff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376258281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26720377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257968,"raw_hex":"e879b89701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376258498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1095514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257968,"raw_hex":"e85ab71000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376258257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257968,"raw_hex":"e86c8b0000","size":5,"soft_fail":false,"target_va":5376294160,"va":5376258463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-45347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376257968,"raw_hex":"e8dd4effff","size":5,"soft_fail":false,"target_va":5376212784,"va":5376258126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":32864,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":32408,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":32380,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":32436,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":559,"record_offset":32808,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":559,"record_offset":32960,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":32492,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":32080,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":559,"record_offset":32464,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":559,"record_offset":32296,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":31896,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":32624,"parent_record_offset":32568,"ancestor_record_offset":31896,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":32596,"parent_record_offset":32568,"ancestor_record_offset":31896,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":32568,"parent_record_offset":32540,"ancestor_record_offset":31896,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":559,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":31896,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":559,"record_offset":32248,"parent_record_offset":32220,"ancestor_record_offset":31896,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":32516,"parent_record_offset":32492,"ancestor_record_offset":31896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":32136,"parent_record_offset":32108,"ancestor_record_offset":31896,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":32108,"parent_record_offset":32080,"ancestor_record_offset":31896,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":32900,"parent_record_offset":32864,"ancestor_record_offset":31896,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":32320,"parent_record_offset":32296,"ancestor_record_offset":31896,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":32924,"parent_record_offset":32900,"ancestor_record_offset":31896,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":32344,"parent_record_offset":32320,"ancestor_record_offset":31896,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$7::create_device::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":559,"record_offset":32168,"parent_record_offset":32080,"ancestor_record_offset":31896,"inlinee":30675,"source_line":741,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":559,"record_offset":32192,"parent_record_offset":32168,"ancestor_record_offset":31896,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::create_device::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 741
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77D3
// 首条位置: {"module":559,"inlinee":30675}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] uuid::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.1.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.1.2/src/lib.rs
//     evidence={"module":559,"record_offset":32192,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":13086,"source_line":397,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":559,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":32168,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":559,"record_offset":32248,"parent_record_offset":32220,"ancestor_record_offset":32168,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::GraphicsDevicesForSurface::order_and_filter_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 754
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":33196,"rva":7549472,"va":5376258592}
// 编译实例 RVAs: 0x733220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141179ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10775661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376258592,"raw_hex":"e86d6ca400","size":5,"soft_fail":false,"target_va":5387034304,"va":5376258638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117e650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10793992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376258592,"raw_hex":"e808b4a400","size":5,"soft_fail":false,"target_va":5387052624,"va":5376258627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":33352,"parent_record_offset":33196,"ancestor_record_offset":33196,"inlinee":20627,"source_line":305,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":33376,"parent_record_offset":33352,"ancestor_record_offset":33196,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111E1
// 首条位置: {"module":1598,"inlinee":70113}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
//     owner=private_source_ownership_candidate; count=28
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1598,"record_offset":6980,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":30668,"source_line":581,"archive":"_pdb/A/modules/01598.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 777
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111B6
// 首条位置: {"module":1593,"inlinee":70070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1593,"record_offset":8028,"parent_record_offset":7344,"ancestor_record_offset":7344,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1593,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":7344,"inlinee":30669,"source_line":604,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1593,"record_offset":7432,"parent_record_offset":7344,"ancestor_record_offset":7344,"inlinee":13795,"source_line":423,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1593,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":7344,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1593,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":7344,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1593,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":7344,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1593,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":7344,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1593,"record_offset":7544,"parent_record_offset":7496,"ancestor_record_offset":7344,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1593,"record_offset":7620,"parent_record_offset":7496,"ancestor_record_offset":7344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1593,"record_offset":8072,"parent_record_offset":8028,"ancestor_record_offset":7344,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1593,"record_offset":8100,"parent_record_offset":8028,"ancestor_record_offset":7344,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1593,"record_offset":7496,"parent_record_offset":7432,"ancestor_record_offset":7344,"inlinee":13797,"source_line":42,"archive":"_pdb/A/modules/01593.jsonl.gz"}
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
// 完整结构及原始类型记录: init.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::AvailableGraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["alloc::string::String", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::string::String", "alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::string::String", "core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>", "tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::string::String", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>,tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$52::fmt<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "core::fmt::impl$52::fmt", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "slice2$", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<rhapsody::raw::init::AvailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>", "constructor": "alloc::slice::impl$9::to_owned", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>", "constructor": "alloc::vec::Vec::retain", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::init::AvailableGraphicsDevice,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by_key::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "i64", "rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>", "alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >", "tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::AvailableGraphicsDeviceInitState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsDevicesForSurface
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >", "constructor": "rhapsody::raw::init::GraphicsDevicesForSurface::new", "arguments": ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsDevicesForSurface>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsDevicesForSurface"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInitError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 40
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::GraphicsInitError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::GraphicsInitError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::init::GraphicsInstance", "enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::sys_common::backtrace::__rust_begin_short_backtrace", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Slot", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<usize>", "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$55::from_iter<std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "alloc::boxed::impl$55::from_iter", "arguments": ["std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::with_capacity<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::with_capacity", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::Builder::spawn", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::impl$0::spawn_unchecked_::closure_env$1", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::SyncSender", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "std::thread::impl$0::spawn_unchecked_::MaybeDangling", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstance
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 35
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::init::GraphicsInstance", "enum2$<rhapsody::raw::init::GraphicsInitError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::sys_common::backtrace::__rust_begin_short_backtrace", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Slot", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<usize>", "std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$55::from_iter<std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "alloc::boxed::impl$55::from_iter", "arguments": ["std::sync::mpmc::array::Slot<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,std::sync::mpmc::array::impl$1::with_capacity::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::with_capacity<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::with_capacity", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tiny_glade::run_game::compute_on_dedicated_thread<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "constructor": "tiny_glade::run_game::compute_on_dedicated_thread", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >", "tiny_glade::run_game::run_game::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::Builder::spawn<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::Builder::spawn", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "constructor": "std::thread::impl$0::spawn_unchecked_::closure_env$1", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpsc::SyncSender", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SyncSender<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >", "constructor": "std::thread::impl$0::spawn_unchecked_::MaybeDangling", "arguments": ["tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>", "rhapsody::raw::init::impl$4::implicit_layers::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::init::GraphicsInstanceLayer,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsInstanceLayer>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$2::view::closure$1::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>", "tg_gpu_selector::vulkan_config::impl$2::view::closure$1::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$2::view::closure$1::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$2::view::closure$1::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayerAllowlist
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,rhapsody::raw::init::GraphicsInstanceLayerAllowlist>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "rhapsody::raw::init::GraphicsInstanceLayerAllowlist"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist>", "constructor": "core::option::Option", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlist"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::init::GraphicsInstanceLayerAllowlist>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlist"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$5::dedup::closure_env$0", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "alloc::vec::Vec::dedup_by", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global", "alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>", "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::MapAccess::next_value<ron::de::CommaSeparated,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "serde::de::MapAccess::next_value", "arguments": ["ron::de::CommaSeparated", "alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "constructor": "ref$", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >", "constructor": "core::slice::sort::merge_sort", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)", "alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$0>,alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global", "tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::GraphicsInstanceLoader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::UnavailableGraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::UnavailableGraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<rhapsody::raw::init::UnavailableGraphicsDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >", "tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::init::impl$4::new::LocationToTry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "constructor": "enum2$", "arguments": ["rhapsody::raw::init::impl$4::new::LocationToTry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<rhapsody::raw::init::impl$4::new::LocationToTry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: init.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: init.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::render::render_config::LastKnownGraphicsDevice>,ref$<country_core::render::render_config::LastKnownGraphicsDevice>)
// 候选参数类型: ["ref$<country_core::render::render_config::LastKnownGraphicsDevice>", "ref$<country_core::render::render_config::LastKnownGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<winit::monitor::VideoMode>,ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>", "ref$<winit::monitor::VideoMode>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::driver_support
// 信息 ID: b8f833056b885d209f574294
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c7760 -> 0x7323d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c9705 -> 0x7323d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_supported @ 0x7322a2 -> 0x7323d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible @ 0x732352 -> 0x7323d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::impl$2::view @ 0x19966ee -> 0x7323d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_gpu_selector::gpu_selection::device_driver_advice @ 0x1998c20 -> 0x7323d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::has_enough_video_memory
// 信息 ID: 156455c49019158116c54e94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=559, inline_record=26276; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible; module=559, inline_record=26524; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1593, inline_record=7400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_compatible
// 信息 ID: bcdb7e5b7b7897049d94ff5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c96f3 -> 0x732340 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=559, inline_record=26192; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_emulated
// 信息 ID: d2c3d90c58c0f72afc21153a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 31 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0x12fd2f8 -> 0x732730 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0x12fda48 -> 0x732730 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_supported; module=559, inline_record=26248; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::is_compatible; module=559, inline_record=26496; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::init::AvailableGraphicsDevice::driver_support; module=559, inline_record=26704; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=6980; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7008; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7092; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7120; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7436; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_old_intel
// 信息 ID: 8aaab353403ceceb39317be7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::AvailableGraphicsDevice::is_supported @ 0x732331 -> 0x732790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDevice::is_supported
// 信息 ID: fbf4dbe4d261817128d0810b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c6e30 -> 0x732290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::AvailableGraphicsDeviceInitState::is_error
// 信息 ID: e25f52e8ea008214755dc4b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x1a90563 -> 0x549fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x1a923d3 -> 0x549fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x1c52eeb -> 0x549fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::mantain_lantern_lights @ 0x1c5d0be -> 0x549fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::available_devices
// 信息 ID: d424bf55c5edaec383631e37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c6664 -> 0x732f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::create_device
// 信息 ID: 65b6c3163df544c6d78f1a44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c78e9 -> 0x732fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices
// 信息 ID: ddfa88bca218d8e7e0d314ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c6712 -> 0x732900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >
// 信息 ID: a34d2bd22be64887cc4c8ce6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["winit::window::Window", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::sync::Arc<winit::window::Window,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::init::GraphicsDevicesForSurface::new -> ["ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >", "ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c61d9 -> 0x3466a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsDevicesForSurface::order_and_filter_devices
// 信息 ID: ea8262a4d8a334cbde3c0336
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c6756 -> 0x733220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstance::new
// 信息 ID: 6f90fc32184c25b59f78ac47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > @ 0x1ae7c -> 0x7304c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayer::vendor
// 信息 ID: abb6606508c96cff95786ff9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::vulkan_config::Controls::new @ 0x8f7891 -> 0x7302b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
// 信息 ID: dca46c98beffc7bf8c2efde7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc12d99c00e7307ec4b5db0b: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] e98aff1265803b4c26f26dec: iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::init::GraphicsInstanceLayer>,tg_gpu_selector::vulkan_config::impl$2::view::closure$1::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x1300296 -> 0x7303e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=6428; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=7080; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8016; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8376; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
// 信息 ID: 8333086485130cf1b5ba08d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 82810ea9d2375ca9b2c82431: iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] e98aff1265803b4c26f26dec: iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<enum2$<core::option::Option<alloc::string::String> >,tg_gpu_selector::vulkan_config::VendorLayers> >,tg_gpu_selector::vulkan_config::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0x12fb196 -> 0x730450 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=7440; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8724; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
// 信息 ID: f88f9cf70682f5ab2afba3a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x244967 -> 0x732200 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=559, inline_record=13020; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLoader::new
// 信息 ID: 31ff2799415ace201f9d9316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::compute_on_dedicated_thread::closure_env$0<enum2$<core::result::Result<rhapsody::raw::init::GraphicsInstance,enum2$<rhapsody::raw::init::GraphicsInitError> > >,tiny_glade::run_game::run_game::closure_env$0>,tuple$<> > @ 0x1ae1f -> 0x730b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x24457e -> 0x730b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::GraphicsInstanceLoader::supports_modern_layer_filtering
// 信息 ID: a68d1a1ffaab797446b14275
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=1132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize
// 信息 ID: 92f24442bbb2e6789a8154a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> > >; module=117, inline_record=43988; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::render::render_config::_::impl$0::deserialize::__Visitor>; module=117, inline_record=99764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: b2df94faaa553d5da8ba358a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<rhapsody::raw::init::_::impl$0::deserialize::__Field> >; module=117, inline_record=6400; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=86244; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<rhapsody::raw::init::_::impl$0::deserialize::__FieldVisitor>; module=207, inline_record=18048; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<rhapsody::raw::init::_::impl$0::deserialize::__FieldVisitor>; module=207, inline_record=18468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: bd20d20f6fd2450a94a7a3bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<rhapsody::raw::init::_::impl$0::deserialize::__Field> >; module=117, inline_record=6248; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=85740; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=85948; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=86104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 90ba9c921804597d025cd2c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=62484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 6b0c85b839b08935babdebd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<rhapsody::raw::init::_::impl$0::deserialize::__Visitor>; module=117, inline_record=85352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::serialize
// 信息 ID: f3b4e2aae20b802a751ba091
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=6, inline_record=48040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 8313654e49a225794cbb12bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::init::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,rhapsody::raw::init::GraphicsInstanceLayerAllowlist> @ 0xa4d4 -> 0x3463d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$10::fmt
// 信息 ID: 27482ca86574c4fa4922d7d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >; module=222, inline_record=6316; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::init::GraphicsInitError> >; module=778, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$14::from
// 信息 ID: 7e997c6e16ba851823e53b8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c87f2 -> 0x733260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$17::clone
// 信息 ID: 1fb9e474584fd58a19bb03ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vulkan_config_main::run_vulkan_configuration_app; module=126, inline_record=13552; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=25408; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=170, inline_record=3580; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=47832; 不是独立调用边。
// [PDB 内联宿主] country_core::game_config::impl$9::clone; module=256, inline_record=14868; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=17624; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=779, inline_record=7500; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::vulkan_config::impl$2::update; module=781, inline_record=5980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$19::eq
// 信息 ID: 64db36608ea942cd6429e660
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=73, inline_record=17044; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::dedup_by<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl$5::dedup::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global> >; module=73, inline_record=17508; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>; module=1980, inline_record=6500; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,tg_gpu_selector::vulkan_config::impl$1::toggle_allowlist_item::closure_env$0>; module=1980, inline_record=7064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$1::contains_layer::closure$0
// 信息 ID: 3820eae58223b5fe83ef3b62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$1::contains_layer", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer; module=559, inline_record=10804; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=6616; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=7204; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$1::contains_vendor::closure$0
// 信息 ID: c6779488545f12b4c09c6d58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$1::contains_vendor", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor; module=559, inline_record=11288; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=7556; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=8840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$21::partial_cmp
// 信息 ID: ca86b954e2422bf6ab20e4c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 397666899118c3873d44ca17: bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >; module=194, inline_record=29324; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >; module=194, inline_record=29660; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >; module=194, inline_record=29992; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >; module=194, inline_record=31220; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >),alloc::slice::stable_sort::closure_env$0<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$1<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$2<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>,alloc::slice::stable_sort::closure_env$3<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)> >; module=194, inline_record=31740; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=194, inline_record=39176; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)>; module=194, inline_record=40116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$23::clone
// 信息 ID: a942ac3942cb3d505ac236c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>; module=73, inline_record=42124; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$10::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>; module=1980, inline_record=15280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$25::fmt
// 信息 ID: 11ae3d63662bc6248923a2ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$27::fmt
// 信息 ID: dc5b08069513553f1147f0c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$29::eq
// 信息 ID: 62ff8397b02bfd88fe179159
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::impl$2::view; module=3235, inline_record=4156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$31::clone
// 信息 ID: f0a0964f34a0369fba5cc899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=35096; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>; module=778, inline_record=1456; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1593, inline_record=7432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$32::fmt
// 信息 ID: 24d4d14794a1a5ccb817c600
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$52::fmt<rhapsody::raw::init::AvailableGraphicsDevice>; module=222, inline_record=7536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$3::new::closure$1
// 信息 ID: 065cf5d7b6fc5db0c080bf5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::init::impl$3::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=559, inline_record=12324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::implicit_layers::closure$0
// 信息 ID: f9b672650a39a115fb20da8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::implicit_layers", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::init::GraphicsInstanceLayer,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$4::implicit_layers::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ash::vk::definitions::LayerProperties>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::GraphicsInstanceLayer,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::LayerProperties,alloc::alloc::Global>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0> > > >; module=21, inline_record=11728; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ash::vk::definitions::LayerProperties>,rhapsody::raw::init::impl$4::implicit_layers::closure_env$0>; module=1604, inline_record=484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::new::closure$0
// 信息 ID: b63663bd053194f3e72582e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLoader::new; module=559, inline_record=16168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$4::new::closure$0::closure$0
// 信息 ID: 0087005ed5c855ee19a89563
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$4::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstanceLoader::new; module=559, inline_record=16560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::available_devices::closure$0
// 信息 ID: 56267fd2d2ec5c2b92e1be2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::available_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::available_devices::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::init::AvailableGraphicsDevice,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::available_devices::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::available_devices::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=1260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::create_device::closure$0
// 信息 ID: e18b30a32d7a26d738f4792a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::create_device", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsDevicesForSurface::create_device; module=559, inline_record=32168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::missing_devices::closure$0
// 信息 ID: 7108439a1dbeeb381de58578
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::missing_devices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices; module=559, inline_record=30096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::new::closure$2
// 信息 ID: fc255b96dbe2efdce10c7959
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::init::impl$7::new", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::new::closure_env$2 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::new::closure_env$2 <- core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::new::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >; module=21, inline_record=19256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$0
// 信息 ID: 4a6690c73e2696ffb28d703a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::init::impl$7::order_and_filter_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0 <- core::slice::sort::insertion_sort_shift_left<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=6920; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7040; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7376; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7496; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7832; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=7964; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=9072; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0>,alloc::slice::stable_sort::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<rhapsody::raw::init::AvailableGraphicsDevice,alloc::slice::impl$0::sort_by_key::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,i64,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$0> > >; module=1598, inline_record=9188; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::init::impl$7::order_and_filter_devices::closure$1
// 信息 ID: d8e51b23087c856f0030057c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::init::impl$7::order_and_filter_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1 <- alloc::vec::Vec::retain<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1 <- alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1593, inline_record=7344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
