// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/save_error.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 60708bb40fcd1a612e70119b42846c6182d642f83d3cd03793825679761946f1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_error.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1235, 1239, 2963, 4865
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2963,"record_offset":264,"rva":37522896,"va":5406232016}
// 编译实例 RVAs: 0x23c8dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21583659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8d5a8b6fe","size":5,"soft_fail":false,"target_va":5384648448,"va":5406232102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21584133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8fba6b6fe","size":5,"soft_fail":false,"target_va":5384648032,"va":5406232160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30554367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e801c72dfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5406233066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32753495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8a9380cfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5406232530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25168230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e89af67ffe","size":5,"soft_fail":false,"target_va":5381064176,"va":5406232401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb3030
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24206818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e81ea28efe","size":5,"soft_fail":false,"target_va":5382025264,"va":5406232077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411780c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19206674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8eeeddafe","size":5,"soft_fail":false,"target_va":5387026624,"va":5406233293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19186973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8e33adbfe","size":5,"soft_fail":false,"target_va":5387045984,"va":5406232952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141193420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19094936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e868a2dcfe","size":5,"soft_fail":false,"target_va":5387138080,"va":5406233011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141194870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19089709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8d3b6dcfe","size":5,"soft_fail":false,"target_va":5387143280,"va":5406232984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141198710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19073730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e83ef5dcfe","size":5,"soft_fail":false,"target_va":5387159312,"va":5406233037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10118931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e813679a00","size":5,"soft_fail":false,"target_va":5416351392,"va":5406232456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21437502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8c2e3b8fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5406232361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10388512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e820849e00","size":5,"soft_fail":false,"target_va":5416621600,"va":5406233083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24624509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8834288fe","size":5,"soft_fail":false,"target_va":5381607616,"va":5406232120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_game::AutoSaveState::last_error_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26195703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8094970fe","size":5,"soft_fail":false,"target_va":5380036368,"va":5406232066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23097468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8848f9ffe","size":5,"soft_fail":false,"target_va":5383134720,"va":5406232183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30026066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406232016,"raw_hex":"e8aed635fe","size":5,"soft_fail":false,"target_va":5376206368,"va":5406232429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2963,"record_offset":576,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2963,"record_offset":828,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2963,"record_offset":424,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":540,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":3380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2963,"record_offset":512,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":2963,"record_offset":480,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2963,"record_offset":3352,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":888,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":2280,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":22024,"source_line":1551,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":1204,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10274,"source_line":1421,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::save_error::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/save_error.rs
//     evidence={"module":2963,"record_offset":3664,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":112389,"source_line":38,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2963,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2963,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2963,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2963,"record_offset":3532,"parent_record_offset":3476,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2963,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":264,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":1040,"parent_record_offset":1004,"ancestor_record_offset":264,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":980,"parent_record_offset":912,"ancestor_record_offset":264,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":264,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2963,"record_offset":856,"parent_record_offset":828,"ancestor_record_offset":264,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2963,"record_offset":3708,"parent_record_offset":3664,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2963,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":264,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":264,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2963,"record_offset":2068,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":264,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":264,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":264,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":2240,"parent_record_offset":1268,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2963,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":264,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2963,"record_offset":2320,"parent_record_offset":2280,"ancestor_record_offset":264,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":1160,"parent_record_offset":1004,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2963,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":264,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1712,"parent_record_offset":1684,"ancestor_record_offset":264,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":264,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1684,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":1544,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":2096,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":1628,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2963,"record_offset":1488,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2963,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":264,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":264,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1952,"parent_record_offset":1844,"ancestor_record_offset":264,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":264,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":264,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":2188,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":264,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":264,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1384,"parent_record_offset":1324,"ancestor_record_offset":264,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":264,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1516,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1844,"parent_record_offset":1432,"ancestor_record_offset":264,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2963,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":264,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2963,"record_offset":5156,"parent_record_offset":3664,"ancestor_record_offset":264,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":264,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":3768,"parent_record_offset":3664,"ancestor_record_offset":264,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":4084,"parent_record_offset":3664,"ancestor_record_offset":264,"inlinee":22024,"source_line":1551,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":264,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2963,"record_offset":3292,"parent_record_offset":2280,"ancestor_record_offset":264,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":952,"parent_record_offset":912,"ancestor_record_offset":264,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::save_error::ui::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/save_error.rs
//     evidence={"module":2963,"record_offset":1240,"parent_record_offset":1204,"ancestor_record_offset":264,"inlinee":70084,"source_line":88,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B705
// 首条位置: {"module":2963,"inlinee":112389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2963,"record_offset":3708,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2963,"record_offset":5156,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":3768,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":4084,"parent_record_offset":3664,"ancestor_record_offset":3664,"inlinee":22024,"source_line":1551,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":3952,"parent_record_offset":3920,"ancestor_record_offset":3664,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3664,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2963,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":3664,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":3920,"parent_record_offset":3884,"ancestor_record_offset":3664,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":3860,"parent_record_offset":3792,"ancestor_record_offset":3664,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2963,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":3664,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2963,"record_offset":3736,"parent_record_offset":3708,"ancestor_record_offset":3664,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2963,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":3664,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":3664,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3664,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2963,"record_offset":4924,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":3664,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":3664,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":5124,"parent_record_offset":4084,"ancestor_record_offset":3664,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3664,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2963,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":3664,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2963,"record_offset":4124,"parent_record_offset":4084,"ancestor_record_offset":3664,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":4040,"parent_record_offset":3884,"ancestor_record_offset":3664,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2963,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":3664,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":4568,"parent_record_offset":4540,"ancestor_record_offset":3664,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":4600,"parent_record_offset":4568,"ancestor_record_offset":3664,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":4540,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":4400,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":4952,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":4484,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2963,"record_offset":4344,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2963,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":3664,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":3664,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":4808,"parent_record_offset":4700,"ancestor_record_offset":3664,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":3664,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":3664,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":5044,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":3664,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3664,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4240,"parent_record_offset":4180,"ancestor_record_offset":3664,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":3664,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4372,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":4700,"parent_record_offset":4288,"ancestor_record_offset":3664,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2963,"record_offset":4152,"parent_record_offset":4084,"ancestor_record_offset":3664,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3664,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2963,"record_offset":5096,"parent_record_offset":4084,"ancestor_record_offset":3664,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":3832,"parent_record_offset":3792,"ancestor_record_offset":3664,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111B8
// 首条位置: {"module":1235,"inlinee":70072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1235,"record_offset":109028,"parent_record_offset":106960,"ancestor_record_offset":106960,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":107004,"parent_record_offset":106960,"ancestor_record_offset":106960,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":107764,"parent_record_offset":107740,"ancestor_record_offset":106960,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":107332,"parent_record_offset":107308,"ancestor_record_offset":106960,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":107416,"parent_record_offset":107392,"ancestor_record_offset":106960,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":107832,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":107564,"parent_record_offset":107536,"ancestor_record_offset":106960,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":107536,"parent_record_offset":107508,"ancestor_record_offset":106960,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":108996,"parent_record_offset":108972,"ancestor_record_offset":106960,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":107356,"parent_record_offset":107332,"ancestor_record_offset":106960,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":107680,"parent_record_offset":107656,"ancestor_record_offset":106960,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":107036,"parent_record_offset":107004,"ancestor_record_offset":106960,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1235,"record_offset":107788,"parent_record_offset":107764,"ancestor_record_offset":106960,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":107476,"parent_record_offset":107448,"ancestor_record_offset":106960,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":107508,"parent_record_offset":107476,"ancestor_record_offset":106960,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":107448,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":107308,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":107860,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":107392,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1235,"record_offset":107252,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1235,"record_offset":107740,"parent_record_offset":107716,"ancestor_record_offset":106960,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":107092,"parent_record_offset":107064,"ancestor_record_offset":106960,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":107716,"parent_record_offset":107608,"ancestor_record_offset":106960,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":107124,"parent_record_offset":107092,"ancestor_record_offset":106960,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107656,"parent_record_offset":107632,"ancestor_record_offset":106960,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107952,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107632,"parent_record_offset":107608,"ancestor_record_offset":106960,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107176,"parent_record_offset":107152,"ancestor_record_offset":106960,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107152,"parent_record_offset":107092,"ancestor_record_offset":106960,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107200,"parent_record_offset":107176,"ancestor_record_offset":106960,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107280,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":107608,"parent_record_offset":107200,"ancestor_record_offset":106960,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":107064,"parent_record_offset":107004,"ancestor_record_offset":106960,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111C4
// 首条位置: {"module":1235,"inlinee":70084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2963,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2963,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1240,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":1240,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2963,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1240,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2963,"record_offset":2068,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":1240,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2963,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1240,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2963,"record_offset":2240,"parent_record_offset":1268,"ancestor_record_offset":1240,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2963,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":1240,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2963,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1240,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2963,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1240,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1712,"parent_record_offset":1684,"ancestor_record_offset":1240,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":1240,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2963,"record_offset":1684,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":1544,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":2096,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2963,"record_offset":1628,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2963,"record_offset":1488,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2963,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1240,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":1240,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1952,"parent_record_offset":1844,"ancestor_record_offset":1240,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2963,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":1240,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1240,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":2188,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1240,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":1240,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1384,"parent_record_offset":1324,"ancestor_record_offset":1240,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1240,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1516,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2963,"record_offset":1844,"parent_record_offset":1432,"ancestor_record_offset":1240,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2963,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":1240,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":184192,"rva":18466528,"va":5387175648}
// 编译实例 RVAs: 0x119c6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2383808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e840a0dbff","size":5,"soft_fail":false,"target_va":5384792224,"va":5387176027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-11497411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e83d9050ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387176110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13691360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e820162fff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387175739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13690980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e89c172fff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387175759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6111545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e8c7bea2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387175716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e82aadfdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387175825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-150335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e8c1b4fdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387175786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e843700000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387175944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e8d3740000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387175800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2381221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e85baadbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387176080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4041290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e8b655c2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387176005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5160818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e88e40b1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387175869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5164565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e8eb31b1ff","size":5,"soft_fail":false,"target_va":5382011344,"va":5387175904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5163753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e81735b1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387175988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10967413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387175648,"raw_hex":"e88ba658ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387175776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":184940,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1239,"record_offset":185336,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":185024,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":184856,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21764,"source_line":2480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184616,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184556,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184676,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21734,"source_line":290,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184796,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184736,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184352,"parent_record_offset":184192,"ancestor_record_offset":184192,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":185224,"parent_record_offset":185200,"ancestor_record_offset":184192,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":185248,"parent_record_offset":185224,"ancestor_record_offset":184192,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":185272,"parent_record_offset":185248,"ancestor_record_offset":184192,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":185200,"parent_record_offset":185144,"ancestor_record_offset":184192,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":184988,"parent_record_offset":184964,"ancestor_record_offset":184192,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":185172,"parent_record_offset":185144,"ancestor_record_offset":184192,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":185144,"parent_record_offset":185120,"ancestor_record_offset":184192,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":185120,"parent_record_offset":185096,"ancestor_record_offset":184192,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":184964,"parent_record_offset":184940,"ancestor_record_offset":184192,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":185048,"parent_record_offset":185024,"ancestor_record_offset":184192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1239,"record_offset":184880,"parent_record_offset":184856,"ancestor_record_offset":184192,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1239,"record_offset":184904,"parent_record_offset":184880,"ancestor_record_offset":184192,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":184464,"parent_record_offset":184404,"ancestor_record_offset":184192,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":184404,"parent_record_offset":184352,"ancestor_record_offset":184192,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":184376,"parent_record_offset":184352,"ancestor_record_offset":184192,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":184436,"parent_record_offset":184404,"ancestor_record_offset":184192,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":184488,"parent_record_offset":184464,"ancestor_record_offset":184192,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184584,"parent_record_offset":184556,"ancestor_record_offset":184192,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":184520,"parent_record_offset":184404,"ancestor_record_offset":184192,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":186724,"rva":18467520,"va":5387176640}
// 编译实例 RVAs: 0x119cac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11498448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8308c50ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387177147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13692486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8ba112fff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387176865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13692107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e835132fff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387176886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6112670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e862baa2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387176841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-153407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8c1a8fdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387176954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e85bb0fdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387176912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8d46b0000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387177079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e86b700000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387176928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2381953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e87fa7dbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387176812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4041969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e80f53c2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387176684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5161949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8233cb1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387177000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5165698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e87e2db1ff","size":5,"soft_fail":false,"target_va":5382011344,"va":5387177037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5164887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e8a930b1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387177122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10968538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387176640,"raw_hex":"e826a258ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387176901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":186876,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":187464,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187224,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187164,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187284,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":21734,"source_line":290,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187404,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187344,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":186960,"parent_record_offset":186724,"ancestor_record_offset":186724,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":187640,"parent_record_offset":187616,"ancestor_record_offset":186724,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":187664,"parent_record_offset":187640,"ancestor_record_offset":186724,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":187688,"parent_record_offset":187664,"ancestor_record_offset":186724,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":187616,"parent_record_offset":187560,"ancestor_record_offset":186724,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":186924,"parent_record_offset":186900,"ancestor_record_offset":186724,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":187588,"parent_record_offset":187560,"ancestor_record_offset":186724,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":187560,"parent_record_offset":187536,"ancestor_record_offset":186724,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":187536,"parent_record_offset":187512,"ancestor_record_offset":186724,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":186900,"parent_record_offset":186876,"ancestor_record_offset":186724,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":187488,"parent_record_offset":187464,"ancestor_record_offset":186724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":187072,"parent_record_offset":187012,"ancestor_record_offset":186724,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":187012,"parent_record_offset":186960,"ancestor_record_offset":186724,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":186984,"parent_record_offset":186960,"ancestor_record_offset":186724,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":187044,"parent_record_offset":187012,"ancestor_record_offset":186724,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":187096,"parent_record_offset":187072,"ancestor_record_offset":186724,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187192,"parent_record_offset":187164,"ancestor_record_offset":186724,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":187128,"parent_record_offset":187012,"ancestor_record_offset":186724,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":177060,"rva":18464096,"va":5387173216}
// 编译实例 RVAs: 0x119bd60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11494948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8dc9950ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387173631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-11494788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e87c9a50ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387173487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18451425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e81f74e6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387173612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17836530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e80ed6effe","size":5,"soft_fail":false,"target_va":5369338848,"va":5387175373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13689904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8d01b2fff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387174283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13689535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8411d2fff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387174314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8577000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e818207dff","size":5,"soft_fail":false,"target_va":5378598624,"va":5387175619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6110078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e882c4a2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387174249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5e620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2349223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85927dcff","size":5,"soft_fail":false,"target_va":5384824352,"va":5387173570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-150857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8b7b2fdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387174404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-148897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85fbafdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387174348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e89b750000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387174576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8707a0000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387174363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c19e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t154507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e88b5b0200","size":5,"soft_fail":false,"target_va":5387327968,"va":5387173456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c5b90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t169976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8f8970200","size":5,"soft_fail":false,"target_va":5387344784,"va":5387174803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423457b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18519910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e866971a01","size":5,"soft_fail":false,"target_va":5405693872,"va":5387173957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29139514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e83aa2bc01","size":5,"soft_fail":false,"target_va":5416312752,"va":5387173233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29178105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8f938bd01","size":5,"soft_fail":false,"target_va":5416353040,"va":5387174930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: save_error.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29883715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e843fdc701","size":5,"soft_fail":false,"target_va":5417057056,"va":5387173336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2378905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e867b3dbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387173764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29447196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e81c54c101","size":5,"soft_fail":false,"target_va":5416620794,"va":5387173593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<ref$<str$> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-56367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8d123ffff","size":5,"soft_fail":false,"target_va":5387117360,"va":5387173722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2549286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8da19d9ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5387173361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29446747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85b52c101","size":5,"soft_fail":false,"target_va":5416621632,"va":5387174880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18519359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e83f951a01","size":5,"soft_fail":false,"target_va":5405694336,"va":5387174972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6905506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85ea196ff","size":5,"soft_fail":false,"target_va":5380267776,"va":5387173277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6187427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85d96a1ff","size":5,"soft_fail":false,"target_va":5380985888,"va":5387173310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4038827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8555fc2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387173542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5159415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e80946b1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387174466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5163179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e85537b1ff","size":5,"soft_fail":false,"target_va":5382011344,"va":5387174518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5162399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8613ab1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387174634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10965970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e82eac58ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387174333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::local_time::system_time_to_local_time
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3892268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387173216,"raw_hex":"e8d49bc4ff","size":5,"soft_fail":false,"target_va":5383281632,"va":5387173895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":178916,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":177860,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$32::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":179840,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":23709,"source_line":2788,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":178040,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1239,"record_offset":179568,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1239,"record_offset":177396,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":177492,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":179656,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":177248,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1239,"record_offset":179596,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":1239,"record_offset":179628,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181092,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181032,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181152,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":21734,"source_line":290,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181272,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181212,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":180828,"parent_record_offset":177060,"ancestor_record_offset":177060,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":178716,"parent_record_offset":178692,"ancestor_record_offset":177060,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":178740,"parent_record_offset":178716,"ancestor_record_offset":177060,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":177740,"parent_record_offset":177716,"ancestor_record_offset":177060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":177764,"parent_record_offset":177740,"ancestor_record_offset":177060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":178764,"parent_record_offset":178740,"ancestor_record_offset":177060,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":177788,"parent_record_offset":177764,"ancestor_record_offset":177060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":178692,"parent_record_offset":178528,"ancestor_record_offset":177060,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":177716,"parent_record_offset":177660,"ancestor_record_offset":177060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":178964,"parent_record_offset":178940,"ancestor_record_offset":177060,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":180024,"parent_record_offset":179996,"ancestor_record_offset":177060,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":178448,"parent_record_offset":178412,"ancestor_record_offset":177060,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":180120,"parent_record_offset":180092,"ancestor_record_offset":177060,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":177688,"parent_record_offset":177660,"ancestor_record_offset":177060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":177660,"parent_record_offset":177636,"ancestor_record_offset":177060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":180092,"parent_record_offset":180056,"ancestor_record_offset":177060,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":180056,"parent_record_offset":180024,"ancestor_record_offset":177060,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":178528,"parent_record_offset":178484,"ancestor_record_offset":177060,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":178484,"parent_record_offset":178448,"ancestor_record_offset":177060,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":177636,"parent_record_offset":177612,"ancestor_record_offset":177060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":178580,"parent_record_offset":178528,"ancestor_record_offset":177060,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":178300,"parent_record_offset":178264,"ancestor_record_offset":177060,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":178336,"parent_record_offset":178300,"ancestor_record_offset":177060,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":178372,"parent_record_offset":178336,"ancestor_record_offset":177060,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":178264,"parent_record_offset":178228,"ancestor_record_offset":177060,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":177896,"parent_record_offset":177860,"ancestor_record_offset":177060,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1239,"record_offset":178228,"parent_record_offset":178192,"ancestor_record_offset":177060,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":179868,"parent_record_offset":179840,"ancestor_record_offset":177060,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":178080,"parent_record_offset":178040,"ancestor_record_offset":177060,"inlinee":15783,"source_line":2973,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":178156,"parent_record_offset":178120,"ancestor_record_offset":177060,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":178120,"parent_record_offset":178080,"ancestor_record_offset":177060,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":178192,"parent_record_offset":178156,"ancestor_record_offset":177060,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":179952,"parent_record_offset":179924,"ancestor_record_offset":177060,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":179896,"parent_record_offset":179868,"ancestor_record_offset":177060,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":180212,"parent_record_offset":179952,"ancestor_record_offset":177060,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":179996,"parent_record_offset":179952,"ancestor_record_offset":177060,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":178412,"parent_record_offset":178372,"ancestor_record_offset":177060,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":177424,"parent_record_offset":177396,"ancestor_record_offset":177060,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":181644,"parent_record_offset":181616,"ancestor_record_offset":177060,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1239,"record_offset":179924,"parent_record_offset":179896,"ancestor_record_offset":177060,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1239,"record_offset":178604,"parent_record_offset":178580,"ancestor_record_offset":177060,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":177972,"parent_record_offset":177932,"ancestor_record_offset":177060,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":178652,"parent_record_offset":178628,"ancestor_record_offset":177060,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1239,"record_offset":177452,"parent_record_offset":177424,"ancestor_record_offset":177060,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1239,"record_offset":178628,"parent_record_offset":178604,"ancestor_record_offset":177060,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1239,"record_offset":182240,"parent_record_offset":182212,"ancestor_record_offset":177060,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":178940,"parent_record_offset":178916,"ancestor_record_offset":177060,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1239,"record_offset":178820,"parent_record_offset":178372,"ancestor_record_offset":177060,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":178844,"parent_record_offset":178820,"ancestor_record_offset":177060,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":177304,"parent_record_offset":177276,"ancestor_record_offset":177060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":180176,"parent_record_offset":179952,"ancestor_record_offset":177060,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":177276,"parent_record_offset":177248,"ancestor_record_offset":177060,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1239,"record_offset":177932,"parent_record_offset":177896,"ancestor_record_offset":177060,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":180940,"parent_record_offset":180880,"ancestor_record_offset":177060,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":180880,"parent_record_offset":180828,"ancestor_record_offset":177060,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":180852,"parent_record_offset":180828,"ancestor_record_offset":177060,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":180912,"parent_record_offset":180880,"ancestor_record_offset":177060,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":180964,"parent_record_offset":180940,"ancestor_record_offset":177060,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":181060,"parent_record_offset":181032,"ancestor_record_offset":177060,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":180996,"parent_record_offset":180880,"ancestor_record_offset":177060,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::save_error::ui::closure$4::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x23937
// 首条位置: {"module":4865,"inlinee":145719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::time::impl$34::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":4865,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":824,"inlinee":31569,"source_line":248,"archive":"_pdb/C/modules/04865.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::SystemTime::intervals
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":4865,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":824,"inlinee":20849,"source_line":84,"archive":"_pdb/C/modules/04865.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$5::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":4865,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":824,"inlinee":31549,"source_line":129,"archive":"_pdb/C/modules/04865.jsonl.gz"}
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
// 完整结构及原始类型记录: save_error.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: save_error.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_error.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui
// 信息 ID: d61953ebfd8224fa9ec4fe60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 512a91430b0e044dadca5407: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)> @ 0x11e6295 -> 0x23c8dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<std::time::Instant> > >)> > @ 0x2304d1c -> 0x23c8dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$0
// 信息 ID: 605eba0826d0af9695961520
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::save_error::ui; module=2963, inline_record=3664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$0::closure$0
// 信息 ID: f2223ebca29b541878d54e44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$0::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::save_error::ui::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::save_error::ui::closure$0::closure_env$0>; module=1235, inline_record=106960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$1
// 信息 ID: 94020c60237b4772b7714e13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::save_error::ui; module=2963, inline_record=1240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$2
// 信息 ID: 4cf139b9a4886bea2affc061
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$2,ref$<str$> > @ 0x1194b27 -> 0x119c6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$3
// 信息 ID: e906a639c217c5b23a4c50c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$3,ref$<str$> > @ 0x11936a8 -> 0x119cac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$4
// 信息 ID: 081900ebe01a0c0065f7288d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::save_error::ui::closure_env$4,ref$<str$> > @ 0x1198998 -> 0x119bd60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::save_error::ui::closure$4::closure$0
// 信息 ID: ba0dc45d14d3213ae6aa4635
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::save_error::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::save_error::ui::closure$4::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=4865, inline_record=824; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >; module=4865, inline_record=992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
