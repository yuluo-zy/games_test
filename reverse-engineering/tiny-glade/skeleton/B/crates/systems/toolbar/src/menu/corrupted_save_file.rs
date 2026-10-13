// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/menu/corrupted_save_file.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c0434a6fca0775313205033513821f393bec53ab865844c7e60b9f680b5b6d8f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: corrupted_save_file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1181, 1205, 2844
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::corrupted_save_file::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2844,"record_offset":256,"rva":32242672,"va":5400951792}
// 编译实例 RVAs: 0x1ebfbf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26275755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e855106ffe","size":5,"soft_fail":false,"target_va":5374676336,"va":5400952086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26222318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e812e16ffe","size":5,"soft_fail":false,"target_va":5374729632,"va":5400951945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15864979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e86deb0dff","size":5,"soft_fail":false,"target_va":5385087344,"va":5400952318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbf460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: corrupted_save_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15731133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e843f60fff","size":5,"soft_fail":false,"target_va":5385221216,"va":5400952344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25748222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e8021d77fe","size":5,"soft_fail":false,"target_va":5375203792,"va":5400952009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10589403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e8db94a100","size":5,"soft_fail":false,"target_va":5411541776,"va":5400952368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25411046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400951792,"raw_hex":"e81a427cfe","size":5,"soft_fail":false,"target_va":5375540928,"va":5400951969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2844,"record_offset":1152,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2844,"record_offset":956,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2844,"record_offset":400,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2844,"record_offset":2676,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1208,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1496,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":12682,"source_line":1296,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2844,"record_offset":1368,"parent_record_offset":1336,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2844,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2844,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2844,"record_offset":1336,"parent_record_offset":1300,"ancestor_record_offset":256,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2844,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":256,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2844,"record_offset":1300,"parent_record_offset":1232,"ancestor_record_offset":256,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2844,"record_offset":604,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2844,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":256,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2844,"record_offset":472,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2844,"record_offset":504,"parent_record_offset":472,"ancestor_record_offset":256,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2844,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":256,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":2844,"record_offset":428,"parent_record_offset":400,"ancestor_record_offset":256,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2844,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2844,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2844,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2844,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2844,"record_offset":2460,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2844,"record_offset":1028,"parent_record_offset":992,"ancestor_record_offset":256,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2844,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":256,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2844,"record_offset":548,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2844,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2844,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2844,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":256,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2844,"record_offset":2636,"parent_record_offset":1568,"ancestor_record_offset":256,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2844,"record_offset":1116,"parent_record_offset":992,"ancestor_record_offset":256,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2844,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2844,"record_offset":1456,"parent_record_offset":1300,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2844,"record_offset":660,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2844,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":256,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2844,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":256,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2844,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":256,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2076,"parent_record_offset":2044,"ancestor_record_offset":256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2016,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":1876,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":2488,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":1960,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2844,"record_offset":1848,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2844,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":256,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":256,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":2344,"parent_record_offset":2176,"ancestor_record_offset":256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1688,"parent_record_offset":1656,"ancestor_record_offset":256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2284,"parent_record_offset":2200,"ancestor_record_offset":256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":256,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2580,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":256,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1716,"parent_record_offset":1656,"ancestor_record_offset":256,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1820,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2176,"parent_record_offset":1764,"ancestor_record_offset":256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2844,"record_offset":1600,"parent_record_offset":1568,"ancestor_record_offset":256,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":256,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":256,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1272,"parent_record_offset":1232,"ancestor_record_offset":256,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::corrupted_save_file::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/corrupted_save_file.rs
//     evidence={"module":2844,"record_offset":1536,"parent_record_offset":1496,"ancestor_record_offset":256,"inlinee":61830,"source_line":30,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::corrupted_save_file::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF186
// 首条位置: {"module":1181,"inlinee":61830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2844,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":1536,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2844,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":1536,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2844,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1536,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2844,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1536,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2844,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1536,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2844,"record_offset":2460,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2844,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":1536,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2844,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":1536,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2844,"record_offset":2636,"parent_record_offset":1568,"ancestor_record_offset":1536,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2844,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1536,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2844,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1536,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":1536,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2076,"parent_record_offset":2044,"ancestor_record_offset":1536,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2844,"record_offset":2016,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":1876,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":2488,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2844,"record_offset":1960,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2844,"record_offset":1848,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2844,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":1536,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":1536,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":1536,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":2344,"parent_record_offset":2176,"ancestor_record_offset":1536,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2844,"record_offset":1688,"parent_record_offset":1656,"ancestor_record_offset":1536,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2284,"parent_record_offset":2200,"ancestor_record_offset":1536,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":1536,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2580,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1536,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1536,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1716,"parent_record_offset":1656,"ancestor_record_offset":1536,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1536,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":1820,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2844,"record_offset":2176,"parent_record_offset":1764,"ancestor_record_offset":1536,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2844,"record_offset":1600,"parent_record_offset":1568,"ancestor_record_offset":1536,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02844.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::corrupted_save_file::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF27C
// 首条位置: {"module":1205,"inlinee":62076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":108964,"parent_record_offset":107692,"ancestor_record_offset":107692,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":107724,"parent_record_offset":107692,"ancestor_record_offset":107692,"inlinee":10725,"source_line":364,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":107812,"parent_record_offset":107692,"ancestor_record_offset":107692,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":108708,"parent_record_offset":108684,"ancestor_record_offset":107692,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":108216,"parent_record_offset":108192,"ancestor_record_offset":107692,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":108300,"parent_record_offset":108276,"ancestor_record_offset":107692,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":108240,"parent_record_offset":108216,"ancestor_record_offset":107692,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":108776,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":108448,"parent_record_offset":108420,"ancestor_record_offset":107692,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":108420,"parent_record_offset":108392,"ancestor_record_offset":107692,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":110536,"parent_record_offset":110512,"ancestor_record_offset":110440,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":108568,"parent_record_offset":108544,"ancestor_record_offset":107692,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":107860,"parent_record_offset":107836,"ancestor_record_offset":107692,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":108732,"parent_record_offset":108708,"ancestor_record_offset":107692,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":108360,"parent_record_offset":108332,"ancestor_record_offset":107692,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":108392,"parent_record_offset":108360,"ancestor_record_offset":107692,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":108332,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":108192,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":108804,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":108276,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":108164,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":108684,"parent_record_offset":108660,"ancestor_record_offset":107692,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":107944,"parent_record_offset":107916,"ancestor_record_offset":107692,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":107972,"parent_record_offset":107944,"ancestor_record_offset":107692,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":108660,"parent_record_offset":108492,"ancestor_record_offset":107692,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":108004,"parent_record_offset":107972,"ancestor_record_offset":107692,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108600,"parent_record_offset":108516,"ancestor_record_offset":107692,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108544,"parent_record_offset":108516,"ancestor_record_offset":107692,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108896,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108516,"parent_record_offset":108492,"ancestor_record_offset":107692,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108056,"parent_record_offset":108032,"ancestor_record_offset":107692,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108032,"parent_record_offset":107972,"ancestor_record_offset":107692,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108080,"parent_record_offset":108056,"ancestor_record_offset":107692,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108136,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":108492,"parent_record_offset":108080,"ancestor_record_offset":107692,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":107916,"parent_record_offset":107888,"ancestor_record_offset":107692,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":107888,"parent_record_offset":107836,"ancestor_record_offset":107692,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":107776,"parent_record_offset":107752,"ancestor_record_offset":107692,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":107836,"parent_record_offset":107812,"ancestor_record_offset":107692,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":108988,"parent_record_offset":108964,"ancestor_record_offset":107692,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":109012,"parent_record_offset":108988,"ancestor_record_offset":107692,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":107752,"parent_record_offset":107724,"ancestor_record_offset":107692,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
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
// 完整结构及原始类型记录: corrupted_save_file.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::corrupted_save_file::MessageScreenState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 32
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::menu::corrupted_save_file::MessageScreenState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: corrupted_save_file.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: corrupted_save_file.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>", "bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui
// 信息 ID: 99cee8dbfe10472973f5daf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d6ded37771fdcd94ae29f6d3: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > @ 0xff8c39 -> 0x1ebfbf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<toolbar::menu::corrupted_save_file::MessageScreenState>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)> > > @ 0x1e08f48 -> 0x1ebfbf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui::closure$0
// 信息 ID: 4bfb0ae423f1221361d5b6d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::corrupted_save_file::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::corrupted_save_file::ui; module=2844, inline_record=1536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::corrupted_save_file::ui::closure$1
// 信息 ID: 037fdcc055e2f4047702c653
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::corrupted_save_file::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::corrupted_save_file::ui::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=1205, inline_record=107692; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=1205, inline_record=109080; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::corrupted_save_file::ui::closure_env$1,ref$<str$> >; module=1205, inline_record=110440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
