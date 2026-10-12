// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_glade_tod_and_weather.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e391ff8e586f3f3d365a9b1c9ace25f34753832e56896eb26c917917ba631675
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_glade_tod_and_weather.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 396, 1362
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":396,"record_offset":5536,"rva":5471488,"va":5374180608}
// 编译实例 RVAs: 0x537d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f2020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2384257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e87f9edbff","size":5,"soft_fail":false,"target_va":5371797536,"va":5374181788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1866810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e8c683e3ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374181013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1861345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e81f99e3ff","size":5,"soft_fail":false,"target_va":5372320512,"va":5374181852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1431082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e8d629eaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374180789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t990341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e8851c0f00","size":5,"soft_fail":false,"target_va":5375171056,"va":5374180710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d42510
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8431349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e8f5a68000","size":5,"soft_fail":false,"target_va":5382612240,"va":5374180886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28798795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e84b6fb701","size":5,"soft_fail":false,"target_va":5402980128,"va":5374181328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3224846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e80e353100","size":5,"soft_fail":false,"target_va":5377405616,"va":5374180765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3259069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374180608,"raw_hex":"e8bdba3100","size":5,"soft_fail":false,"target_va":5377439776,"va":5374180702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":396,"record_offset":6976,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":7036,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":396,"record_offset":6932,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":5732,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":8536,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_time_of_day::load_default_tod
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_time_of_day.rs
//     evidence={"module":396,"record_offset":6820,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":22682,"source_line":10,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":396,"record_offset":6016,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":5984,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":5844,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":5536,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":6432,"parent_record_offset":6376,"ancestor_record_offset":5536,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":5536,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":5536,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":396,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":5536,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":396,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":5536,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":6216,"parent_record_offset":6016,"ancestor_record_offset":5536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":396,"record_offset":6884,"parent_record_offset":6820,"ancestor_record_offset":5536,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5536,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":8560,"parent_record_offset":8536,"ancestor_record_offset":5536,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5536,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":396,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5536,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":396,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":5536,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":396,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":5536,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":396,"record_offset":6856,"parent_record_offset":6820,"ancestor_record_offset":5536,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":5808,"parent_record_offset":5756,"ancestor_record_offset":5536,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":5536,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5536,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":396,"record_offset":6048,"parent_record_offset":6016,"ancestor_record_offset":5536,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_tod_and_weather::load_snow_particles
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":396,"record_offset":9352,"rva":5472800,"va":5374181920}
// 编译实例 RVAs: 0x538220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1557059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8bd3de8ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374182078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1432487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e85924eaff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374182178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1432916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8ac22eaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374182623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1549083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e81ba31700","size":5,"soft_fail":false,"target_va":5375731504,"va":5374182416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1805743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8af8d1b00","size":5,"soft_fail":false,"target_va":5375988432,"va":5374182684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8638136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8b8ce8300","size":5,"soft_fail":false,"target_va":5382820192,"va":5374182051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28795588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8c462b701","size":5,"soft_fail":false,"target_va":5402978144,"va":5374182551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10195572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e874929b00","size":5,"soft_fail":false,"target_va":5384378096,"va":5374182519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28796310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e89665b701","size":5,"soft_fail":false,"target_va":5402978880,"va":5374182565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t383725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e8edda0500","size":5,"soft_fail":false,"target_va":5374566080,"va":5374182350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1509978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e85a0a1700","size":5,"soft_fail":false,"target_va":5375692416,"va":5374182433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3184163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374181920,"raw_hex":"e823963000","size":5,"soft_fail":false,"target_va":5377366224,"va":5374182056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":396,"record_offset":9592,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":396,"record_offset":10840,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":10904,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":396,"record_offset":10784,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":396,"record_offset":9676,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":9536,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":396,"record_offset":10140,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":396,"record_offset":10876,"parent_record_offset":9352,"ancestor_record_offset":9352,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":9352,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":9352,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":9352,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":10536,"parent_record_offset":10508,"ancestor_record_offset":9352,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":11056,"parent_record_offset":10956,"ancestor_record_offset":9352,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10988,"parent_record_offset":10956,"ancestor_record_offset":9352,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":9352,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9352,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10656,"parent_record_offset":10480,"ancestor_record_offset":9352,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":10956,"parent_record_offset":10928,"ancestor_record_offset":9352,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10416,"parent_record_offset":10388,"ancestor_record_offset":9352,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10388,"parent_record_offset":10360,"ancestor_record_offset":9352,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":9352,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":9352,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":9352,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":396,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":9352,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":396,"record_offset":10248,"parent_record_offset":10204,"ancestor_record_offset":9352,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":396,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":9352,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":10452,"parent_record_offset":10416,"ancestor_record_offset":9352,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":9352,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":396,"record_offset":9972,"parent_record_offset":9944,"ancestor_record_offset":9352,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":396,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":9352,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":396,"record_offset":10000,"parent_record_offset":9944,"ancestor_record_offset":9352,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":396,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":9352,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":396,"record_offset":9640,"parent_record_offset":9616,"ancestor_record_offset":9352,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":396,"record_offset":10048,"parent_record_offset":10024,"ancestor_record_offset":9352,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":396,"record_offset":11020,"parent_record_offset":10988,"ancestor_record_offset":9352,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":396,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":9352,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":396,"record_offset":10692,"parent_record_offset":10416,"ancestor_record_offset":9352,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":9352,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":9352,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9352,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":10204,"parent_record_offset":10140,"ancestor_record_offset":9352,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":10176,"parent_record_offset":10140,"ancestor_record_offset":9352,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":396,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":9352,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":396,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":9352,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":396,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9352,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":396,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":9352,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":396,"record_offset":9944,"parent_record_offset":9796,"ancestor_record_offset":9352,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":396,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":9352,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":396,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":9352,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":396,"record_offset":9876,"parent_record_offset":9848,"ancestor_record_offset":9352,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":396,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":9352,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":396,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":9352,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_tod_and_weather::load_snow_particles::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF752
// 首条位置: {"module":1362,"inlinee":63314}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_glade_tod_and_weather::load_rain_particles
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":396,"record_offset":11128,"rva":5473584,"va":5374182704}
// 编译实例 RVAs: 0x538530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1433190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e89a21eaff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374182881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t518415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e80fe90700","size":5,"soft_fail":false,"target_va":5374701184,"va":5374182764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1548363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e84ba01700","size":5,"soft_fail":false,"target_va":5375731504,"va":5374183136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_glade_tod_and_weather.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1805039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e8ef8a1b00","size":5,"soft_fail":false,"target_va":5375988432,"va":5374183388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28794814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e8be5fb701","size":5,"soft_fail":false,"target_va":5402978144,"va":5374183325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3180895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e85f893000","size":5,"soft_fail":false,"target_va":5377364336,"va":5374183436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10199808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e800a39b00","size":5,"soft_fail":false,"target_va":5384383056,"va":5374183243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28795536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e89062b701","size":5,"soft_fail":false,"target_va":5402978880,"va":5374183339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28796815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e88f67b701","size":5,"soft_fail":false,"target_va":5402980128,"va":5374183308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t383005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e81dd80500","size":5,"soft_fail":false,"target_va":5374566080,"va":5374183070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1509258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182704,"raw_hex":"e88a071700","size":5,"soft_fail":false,"target_va":5375692416,"va":5374183153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":396,"record_offset":12072,"parent_record_offset":11128,"ancestor_record_offset":11128,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":396,"record_offset":11300,"parent_record_offset":11128,"ancestor_record_offset":11128,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":396,"record_offset":12016,"parent_record_offset":11128,"ancestor_record_offset":11128,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":396,"record_offset":11372,"parent_record_offset":11128,"ancestor_record_offset":11128,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":396,"record_offset":12108,"parent_record_offset":11128,"ancestor_record_offset":11128,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11128,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":11816,"parent_record_offset":11792,"ancestor_record_offset":11128,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":11768,"parent_record_offset":11740,"ancestor_record_offset":11128,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":11740,"parent_record_offset":11712,"ancestor_record_offset":11128,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":11712,"parent_record_offset":11684,"ancestor_record_offset":11128,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":11888,"parent_record_offset":11712,"ancestor_record_offset":11128,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":11648,"parent_record_offset":11620,"ancestor_record_offset":11128,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":11620,"parent_record_offset":11592,"ancestor_record_offset":11128,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":11564,"parent_record_offset":11536,"ancestor_record_offset":11128,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":11592,"parent_record_offset":11564,"ancestor_record_offset":11128,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":11536,"parent_record_offset":11508,"ancestor_record_offset":11128,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":396,"record_offset":11508,"parent_record_offset":11480,"ancestor_record_offset":11128,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":396,"record_offset":11480,"parent_record_offset":11436,"ancestor_record_offset":11128,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":396,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11128,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":11684,"parent_record_offset":11648,"ancestor_record_offset":11128,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":396,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":11128,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":396,"record_offset":11924,"parent_record_offset":11648,"ancestor_record_offset":11128,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":11340,"parent_record_offset":11300,"ancestor_record_offset":11128,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":11844,"parent_record_offset":11816,"ancestor_record_offset":11128,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":11436,"parent_record_offset":11372,"ancestor_record_offset":11128,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":396,"record_offset":11408,"parent_record_offset":11372,"ancestor_record_offset":11128,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00396.jsonl.gz"}
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
// 完整结构及原始类型记录: load_glade_tod_and_weather.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_glade_tod_and_weather.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_glade_tod_and_weather.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather
// 信息 ID: 62c971fcf48602b1de568b47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5cfc7ffa8158e7c77ea39dca: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0xe7a4b1 -> 0x537d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0xf1868f -> 0x537d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_tod_and_weather::load_rain_particles
// 信息 ID: e97a758ab057ce97e9699e72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c385d702ff22357472f39e1c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > @ 0xe7ac37 -> 0x538530 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> @ 0xf17a1c -> 0x538530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_tod_and_weather::load_snow_particles
// 信息 ID: 780e8006f8696b3cb2efd23a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86671920392378d5a75ffddb: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0xe6430e -> 0x538220 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0xf17ef2 -> 0x538220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_glade_tod_and_weather::load_snow_particles::closure$0
// 信息 ID: 1c69038b349447be0356cf49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_glade_tod_and_weather::load_snow_particles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
