// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/photomode/setup_photomode_toolbar.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 12a0fa5aff5f73dcb706a25635db6cdcf1f1441f996965311eb2f84c8c1dc2f8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: setup_photomode_toolbar.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 4526, 4527
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
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4527,"record_offset":256,"rva":39995104,"va":5408704224}
// 编译实例 RVAs: 0x26246e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=28
//     target_aliases=1 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33622835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e8cdf4fefd","size":5,"soft_fail":false,"target_va":5375081488,"va":5408704318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33629042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e88edcfefd","size":5,"soft_fail":false,"target_va":5375081504,"va":5408710541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa19f0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23610866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e80eba97fe","size":5,"soft_fail":false,"target_va":5385099760,"va":5408710621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa3670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23602570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e876da97fe","size":5,"soft_fail":false,"target_va":5385107056,"va":5408709621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140faa6e0
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23569910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e80a5a98fe","size":5,"soft_fail":false,"target_va":5385135840,"va":5408705745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142623cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e829e1ffff","size":5,"soft_fail":false,"target_va":5408701616,"va":5408709506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=31
//     evidence={"assembly":"\tcallq\t2831261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e89d332b00","size":5,"soft_fail":false,"target_va":5411540992,"va":5408709726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-28408646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e8ba844efe","size":5,"soft_fail":false,"target_va":5380300912,"va":5408709553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8227319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e8097682ff","size":5,"soft_fail":false,"target_va":5400482272,"va":5408709586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::link_up_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8225684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e86c7c82ff","size":5,"soft_fail":false,"target_va":5400483968,"va":5408709647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8223357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408704224,"raw_hex":"e8838582ff","size":5,"soft_fail":false,"target_va":5400480944,"va":5408704296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4527,"record_offset":6328,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4527,"record_offset":5796,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4527,"record_offset":5852,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4527,"record_offset":2332,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::button
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4527,"record_offset":884,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":133189,"source_line":85,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4527,"record_offset":1924,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":100118,"source_line":127,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":256,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":948,"parent_record_offset":920,"ancestor_record_offset":256,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":464,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4527,"record_offset":6224,"parent_record_offset":6132,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4527,"record_offset":920,"parent_record_offset":884,"ancestor_record_offset":256,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4527,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4527,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4527,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4527,"record_offset":6192,"parent_record_offset":6160,"ancestor_record_offset":256,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4527,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4527,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4527,"record_offset":5940,"parent_record_offset":5852,"ancestor_record_offset":256,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4527,"record_offset":5968,"parent_record_offset":5852,"ancestor_record_offset":256,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4527,"record_offset":5904,"parent_record_offset":5852,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4527,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":256,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4527,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":256,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/04527.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20844
// 首条位置: {"module":4526,"inlinee":133188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":5904,"parent_record_offset":5776,"ancestor_record_offset":5776,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":6052,"parent_record_offset":6024,"ancestor_record_offset":5776,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":5776,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":6112,"parent_record_offset":6080,"ancestor_record_offset":5776,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4526,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":5776,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4526,"record_offset":6024,"parent_record_offset":5988,"ancestor_record_offset":5776,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":5776,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":5776,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/04526.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":4526,"record_offset":5932,"parent_record_offset":5904,"ancestor_record_offset":5776,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/04526.jsonl.gz"}
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
// 完整结构及原始类型记录: setup_photomode_toolbar.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: setup_photomode_toolbar.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: setup_photomode_toolbar.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 信息 ID: 51297808e0f1a4d84af7e9e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::toolbar_setup @ 0x234701a -> 0x26246e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 信息 ID: da2c643680967f68d3fae349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; module=4526, inline_record=5776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
