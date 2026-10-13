// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/ui_color_gates.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d78297e07986edcf3f5a87cc660a5995c25c568fba6646de4100a2607922eb59
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_gates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3749, 3766
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3766,"record_offset":2488,"rva":36372080,"va":5405081200}
// 编译实例 RVAs: 0x22afe70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30001077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e84b3836fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405082560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30001210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8c63736fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405082709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14228fd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-132195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e89dfbfdff","size":5,"soft_fail":false,"target_va":5404949872,"va":5405082062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e822ce0000","size":5,"soft_fail":false,"target_va":5405135136,"va":5405082361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6764070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e826366700","size":5,"soft_fail":false,"target_va":5411847168,"va":5405083093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6457206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e876876200","size":5,"soft_fail":false,"target_va":5411540992,"va":5405083781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-22242030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8129dacfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5405082233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-30728601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8671e2bfe","size":5,"soft_fail":false,"target_va":5374354992,"va":5405083588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6686176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8e0056600","size":5,"soft_fail":false,"target_va":5411768304,"va":5405082123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6457130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e82a876200","size":5,"soft_fail":false,"target_va":5411540963,"va":5405083828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6458007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8978a6200","size":5,"soft_fail":false,"target_va":5411541776,"va":5405083764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26750544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8b0d167fe","size":5,"soft_fail":false,"target_va":5378331360,"va":5405081899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25855520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8e07975fe","size":5,"soft_fail":false,"target_va":5379226416,"va":5405081931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26747283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e86dde67fe","size":5,"soft_fail":false,"target_va":5378336240,"va":5405083518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26746015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e861e367fe","size":5,"soft_fail":false,"target_va":5378336384,"va":5405082394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26244305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e82f8b6ffe","size":5,"soft_fail":false,"target_va":5378838032,"va":5405082332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27339606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e8aad45efe","size":5,"soft_fail":false,"target_va":5377743024,"va":5405082625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24655020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405081200,"raw_hex":"e854cb87fe","size":5,"soft_fail":false,"target_va":5380427664,"va":5405082679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3766,"record_offset":6396,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3766,"record_offset":6620,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":4108,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":8708,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":4136,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":5912,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3766,"record_offset":3008,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3766,"record_offset":4548,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3766,"record_offset":6480,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3766,"record_offset":7148,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3766,"record_offset":5876,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":6836,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3766,"record_offset":7064,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3766,"record_offset":4396,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3766,"record_offset":2676,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3766,"record_offset":3360,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":2488,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":2488,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":2488,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":2488,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":6708,"parent_record_offset":6676,"ancestor_record_offset":2488,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":6988,"parent_record_offset":6932,"ancestor_record_offset":2488,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3766,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":2488,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3766,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":2488,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":3200,"parent_record_offset":3168,"ancestor_record_offset":2488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6036,"parent_record_offset":6008,"ancestor_record_offset":2488,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":8908,"parent_record_offset":8876,"ancestor_record_offset":2488,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":2488,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":2488,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":2488,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6104,"parent_record_offset":6068,"ancestor_record_offset":2488,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6068,"parent_record_offset":6036,"ancestor_record_offset":2488,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":8972,"parent_record_offset":8940,"ancestor_record_offset":2488,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":8940,"parent_record_offset":8908,"ancestor_record_offset":2488,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":2488,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":9008,"parent_record_offset":8972,"ancestor_record_offset":2488,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":8840,"parent_record_offset":8808,"ancestor_record_offset":2488,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":8808,"parent_record_offset":8740,"ancestor_record_offset":2488,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":3168,"parent_record_offset":3124,"ancestor_record_offset":2488,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":2488,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":2488,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":5968,"parent_record_offset":5940,"ancestor_record_offset":2488,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":3124,"parent_record_offset":3080,"ancestor_record_offset":2488,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":6008,"parent_record_offset":5968,"ancestor_record_offset":2488,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":8876,"parent_record_offset":8840,"ancestor_record_offset":2488,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":2488,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3766,"record_offset":6356,"parent_record_offset":5968,"ancestor_record_offset":2488,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3766,"record_offset":5940,"parent_record_offset":5912,"ancestor_record_offset":2488,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3766,"record_offset":4944,"parent_record_offset":4612,"ancestor_record_offset":2488,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3766,"record_offset":3324,"parent_record_offset":3036,"ancestor_record_offset":2488,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3766,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":2488,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3766,"record_offset":4612,"parent_record_offset":4572,"ancestor_record_offset":2488,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3766,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":2488,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3766,"record_offset":3080,"parent_record_offset":3036,"ancestor_record_offset":2488,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3766,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":2488,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3766,"record_offset":5416,"parent_record_offset":4572,"ancestor_record_offset":2488,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3766,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":2488,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3766,"record_offset":8560,"parent_record_offset":7260,"ancestor_record_offset":2488,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3766,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":2488,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3766,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":2488,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3766,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":2488,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3766,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":2488,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3766,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":2488,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3766,"record_offset":5492,"parent_record_offset":5464,"ancestor_record_offset":2488,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3766,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":2488,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3766,"record_offset":5156,"parent_record_offset":4572,"ancestor_record_offset":2488,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3766,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":2488,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3766,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":2488,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3766,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":2488,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3766,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":2488,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3766,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":2488,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3766,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":2488,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3766,"record_offset":5004,"parent_record_offset":4976,"ancestor_record_offset":2488,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3766,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":2488,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3766,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2488,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3766,"record_offset":2924,"parent_record_offset":2892,"ancestor_record_offset":2488,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3766,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":2488,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3766,"record_offset":6300,"parent_record_offset":5968,"ancestor_record_offset":2488,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3766,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2488,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3766,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":2488,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3766,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":2488,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3766,"record_offset":9832,"parent_record_offset":9804,"ancestor_record_offset":2488,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3766,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":2488,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3766,"record_offset":3600,"parent_record_offset":3568,"ancestor_record_offset":2488,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3766,"record_offset":4888,"parent_record_offset":4612,"ancestor_record_offset":2488,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3766,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":2488,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3766,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":2488,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3766,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":2488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3766,"record_offset":5652,"parent_record_offset":5516,"ancestor_record_offset":2488,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3766,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":2488,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3766,"record_offset":2960,"parent_record_offset":2808,"ancestor_record_offset":2488,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3766,"record_offset":9248,"parent_record_offset":8840,"ancestor_record_offset":2488,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":2488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3766,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":2488,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3766,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":2488,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3766,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":2488,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3766,"record_offset":4432,"parent_record_offset":4396,"ancestor_record_offset":2488,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":2488,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":3288,"parent_record_offset":3124,"ancestor_record_offset":2488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3766,"record_offset":4644,"parent_record_offset":4612,"ancestor_record_offset":2488,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3766,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":2488,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3766,"record_offset":4684,"parent_record_offset":4644,"ancestor_record_offset":2488,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3766,"record_offset":4332,"parent_record_offset":4160,"ancestor_record_offset":2488,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3766,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":2488,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3766,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":2488,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3766,"record_offset":4460,"parent_record_offset":4396,"ancestor_record_offset":2488,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3766,"record_offset":4976,"parent_record_offset":4572,"ancestor_record_offset":2488,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3766,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":2488,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3766,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":2488,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3766,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":2488,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3766,"record_offset":3528,"parent_record_offset":3488,"ancestor_record_offset":2488,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3766,"record_offset":3568,"parent_record_offset":3528,"ancestor_record_offset":2488,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3766,"record_offset":3676,"parent_record_offset":3488,"ancestor_record_offset":2488,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3766,"record_offset":3488,"parent_record_offset":3440,"ancestor_record_offset":2488,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3766,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":2488,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3766,"record_offset":2892,"parent_record_offset":2836,"ancestor_record_offset":2488,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3766,"record_offset":3768,"parent_record_offset":3440,"ancestor_record_offset":2488,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3766,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":2488,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3766,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":2488,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":2488,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":4016,"parent_record_offset":3384,"ancestor_record_offset":2488,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":3868,"parent_record_offset":3440,"ancestor_record_offset":2488,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":2808,"parent_record_offset":2748,"ancestor_record_offset":2488,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":2488,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":2488,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":2780,"parent_record_offset":2748,"ancestor_record_offset":2488,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":2488,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3766,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":2488,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3766,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":2488,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DA61
// 首条位置: {"module":3749,"inlinee":121441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3749,"record_offset":4920,"parent_record_offset":4848,"ancestor_record_offset":4848,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3749,"record_offset":5984,"parent_record_offset":4848,"ancestor_record_offset":4848,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3749,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":4848,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3749,"record_offset":5192,"parent_record_offset":5164,"ancestor_record_offset":4848,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3749,"record_offset":5132,"parent_record_offset":4948,"ancestor_record_offset":4848,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3749,"record_offset":5164,"parent_record_offset":4920,"ancestor_record_offset":4848,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3749,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4848,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3749,"record_offset":5012,"parent_record_offset":4972,"ancestor_record_offset":4848,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3749,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":4848,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3749,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4848,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3749,"record_offset":5044,"parent_record_offset":4972,"ancestor_record_offset":4848,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$0::positions::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":3749,"record_offset":5220,"parent_record_offset":5192,"ancestor_record_offset":4848,"inlinee":30851,"source_line":154,"archive":"_pdb/B/modules/03749.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DAD2
// 首条位置: {"module":3766,"inlinee":121554}
// 编译实例 RVAs: 0xed2660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e8771574ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5384251028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9169671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e8f91474ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5384251170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27289859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e80369a001","size":5,"soft_fail":false,"target_va":5411540992,"va":5384251128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall_and_roof::color_cursor_icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14454346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384250976,"raw_hex":"e84a8edc00","size":5,"soft_fail":false,"target_va":5398705424,"va":5384251073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":836,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":980,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":1064,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":804,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":804,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":804,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":804,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":804,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3766,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":804,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":804,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":804,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":804,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3766,"record_offset":12152,"rva":36375456,"va":5405084576}
// 编译实例 RVAs: 0x22b0ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30003160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e8283036fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405084643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30003651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e83d2e36fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405085150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6762375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e8872f6700","size":5,"soft_fail":false,"target_va":5411847168,"va":5405084788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6455892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e854826200","size":5,"soft_fail":false,"target_va":5411540992,"va":5405085095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6683244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e86cfa6500","size":5,"soft_fail":false,"target_va":5411768304,"va":5405085055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6455880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e848826200","size":5,"soft_fail":false,"target_va":5411540963,"va":5405085078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27340832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e8e0cf5efe","size":5,"soft_fail":false,"target_va":5377744016,"va":5405084843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27340675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e87dd05efe","size":5,"soft_fail":false,"target_va":5377744208,"va":5405084878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27341552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405084576,"raw_hex":"e810cd5efe","size":5,"soft_fail":false,"target_va":5377743280,"va":5405084827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":12308,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":13352,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":12456,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3766,"record_offset":13048,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":13436,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":12152,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":12152,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":12152,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":12340,"parent_record_offset":12308,"ancestor_record_offset":12152,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3766,"record_offset":13484,"parent_record_offset":13460,"ancestor_record_offset":12152,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3766,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":12152,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":12152,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":12640,"parent_record_offset":12612,"ancestor_record_offset":12152,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12152,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":13260,"parent_record_offset":13228,"ancestor_record_offset":12152,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":12152,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":12700,"parent_record_offset":12668,"ancestor_record_offset":12152,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":12152,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3766,"record_offset":12736,"parent_record_offset":12700,"ancestor_record_offset":12152,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":12152,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":12152,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3766,"record_offset":12548,"parent_record_offset":12484,"ancestor_record_offset":12152,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":12520,"parent_record_offset":12484,"ancestor_record_offset":12152,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":12152,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":13172,"parent_record_offset":13136,"ancestor_record_offset":12152,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":12612,"parent_record_offset":12576,"ancestor_record_offset":12152,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3766,"record_offset":12484,"parent_record_offset":12456,"ancestor_record_offset":12152,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3766,"record_offset":13104,"parent_record_offset":13076,"ancestor_record_offset":12152,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3766,"record_offset":13136,"parent_record_offset":13104,"ancestor_record_offset":12152,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3766,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":12152,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3766,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":12152,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3766,"record_offset":12784,"parent_record_offset":12760,"ancestor_record_offset":12152,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3766,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":12152,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3766,"record_offset":12808,"parent_record_offset":12784,"ancestor_record_offset":12152,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3766,"record_offset":12832,"parent_record_offset":12808,"ancestor_record_offset":12152,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3766,"record_offset":12976,"parent_record_offset":12576,"ancestor_record_offset":12152,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03766.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3766,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":12152,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03766.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_gates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_gates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_gates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::WoodenGate>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates
// 信息 ID: 3f868107b3b442ca91c49bea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bb5ea3d34e69a431a6940b61: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> > > @ 0x1c924f1 -> 0x22afe70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::writer::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> > @ 0x1c9dc29 -> 0x22afe70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$0
// 信息 ID: 59d35f1ad92c206b670daef4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; module=3749, inline_record=4848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$1
// 信息 ID: 26752fa242ad95aaa6e06d79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$1 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=3766, inline_record=804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$2
// 信息 ID: 8dacae13aec631f5719f51fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ptr::drop_in_place<system_color::ui_color_gates::ui_color_gates::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x22afdf3 -> 0x22b0ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
