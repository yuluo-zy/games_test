// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/toolbar_startup.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6e36c5959046e9efdc2ad90cfa2d4142e07181ee05cebb507432f5287cb85ca2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_startup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3912, 4526
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
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::toolbar_setup
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3912,"record_offset":256,"rva":36984976,"va":5405694096}
// 编译实例 RVAs: 0x2345890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30612846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e892e22cfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405694329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30612769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8dfe22cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405694268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa19f0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20601890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8dea3c5fe","size":5,"soft_fail":false,"target_va":5385099760,"va":5405701645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa3670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20592954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8c6c6c5fe","size":5,"soft_fail":false,"target_va":5385107056,"va":5405700005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa4380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20583828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e86ceac5fe","size":5,"soft_fail":false,"target_va":5385110400,"va":5405694223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140faa6e0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20565554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8ce31c6fe","size":5,"soft_fail":false,"target_va":5385135840,"va":5405701389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe7410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20309193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8371bcafe","size":5,"soft_fail":false,"target_va":5385384976,"va":5405694164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426235d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2999964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e89cc62d00","size":5,"soft_fail":false,"target_va":5408699856,"va":5405699887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6152874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8aae25d00","size":5,"soft_fail":false,"target_va":5411847168,"va":5405694289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t5845357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e86d315900","size":5,"soft_fail":false,"target_va":5411540992,"va":5405695630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5629825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e881e75500","size":5,"soft_fail":false,"target_va":5411330256,"va":5405700426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20344218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e86692c9fe","size":5,"soft_fail":false,"target_va":5385356032,"va":5405700245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25399023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e811717cfe","size":5,"soft_fail":false,"target_va":5380300912,"va":5405699930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3004097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8c1d62d00","size":5,"soft_fail":false,"target_va":5408704224,"va":5405700122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5217703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e85962b0ff","size":5,"soft_fail":false,"target_va":5400482272,"va":5405699970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::link_up_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5216071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e8b968b0ff","size":5,"soft_fail":false,"target_va":5400483968,"va":5405700034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5213368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e84873b0ff","size":5,"soft_fail":false,"target_va":5400480944,"va":5405694307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20551630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405694096,"raw_hex":"e83268c6fe","size":5,"soft_fail":false,"target_va":5385148528,"va":5405700153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3912,"record_offset":424,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3912,"record_offset":7756,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9431,"source_line":851,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":480,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3912,"record_offset":396,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20106,"source_line":49,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_startup::setup_toolbar
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_startup.rs
//     evidence={"module":3912,"record_offset":1040,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":122679,"source_line":57,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":256,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1568,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1600,"parent_record_offset":1568,"ancestor_record_offset":256,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":672,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3912,"record_offset":6464,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":256,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":256,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":7932,"parent_record_offset":7900,"ancestor_record_offset":256,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":256,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":7844,"parent_record_offset":7812,"ancestor_record_offset":256,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":5932,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3912,"record_offset":5988,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3912,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":256,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3912,"record_offset":7784,"parent_record_offset":7756,"ancestor_record_offset":256,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3912,"record_offset":7812,"parent_record_offset":7784,"ancestor_record_offset":256,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3912,"record_offset":640,"parent_record_offset":608,"ancestor_record_offset":256,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":508,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3912,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6076,"parent_record_offset":5988,"ancestor_record_offset":256,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6104,"parent_record_offset":5988,"ancestor_record_offset":256,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6040,"parent_record_offset":5988,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":256,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":256,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3912,"record_offset":1872,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":100118,"source_line":127,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::setup_toolbar
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DF37
// 首条位置: {"module":3912,"inlinee":122679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1568,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3912,"record_offset":6464,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":5932,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3912,"record_offset":5988,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":6200,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3912,"record_offset":1872,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":100118,"source_line":127,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":1040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":1600,"parent_record_offset":1568,"ancestor_record_offset":1040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":6388,"parent_record_offset":6296,"ancestor_record_offset":1040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":6324,"parent_record_offset":6296,"ancestor_record_offset":1040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":1040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":1040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3912,"record_offset":6356,"parent_record_offset":6324,"ancestor_record_offset":1040,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":1040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3912,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":1040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6076,"parent_record_offset":5988,"ancestor_record_offset":1040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6104,"parent_record_offset":5988,"ancestor_record_offset":1040,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":6040,"parent_record_offset":5988,"ancestor_record_offset":1040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":1040,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1040,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::setup_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20840
// 首条位置: {"module":4526,"inlinee":133184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":2252,"parent_record_offset":2188,"ancestor_record_offset":2188,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":2404,"parent_record_offset":2376,"ancestor_record_offset":2188,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2188,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":2464,"parent_record_offset":2432,"ancestor_record_offset":2188,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":2188,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4526,"record_offset":2376,"parent_record_offset":2340,"ancestor_record_offset":2188,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":2308,"parent_record_offset":2280,"ancestor_record_offset":2188,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":2340,"parent_record_offset":2308,"ancestor_record_offset":2188,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":2188,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/04526.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_startup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_startup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_startup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar
// 信息 ID: 26dc4fbb9c597490cd138ae4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_startup::toolbar_setup; module=3912, inline_record=1040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar::closure$0
// 信息 ID: 0659433c0da332d3dc87aa4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_startup::setup_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_startup::setup_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; module=4526, inline_record=2188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::toolbar_setup
// 信息 ID: 9547a0dabbf1558ca70ed9f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c1d24fb5ae7f95d1a0f0763: void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)> > > @ 0x1e0f0eb -> 0x2345890 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)> > @ 0x1e5f487 -> 0x2345890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
