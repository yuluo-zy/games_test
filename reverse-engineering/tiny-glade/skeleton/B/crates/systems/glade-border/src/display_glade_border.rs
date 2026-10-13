// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/glade-border/src/display_glade_border.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9efb220db579f4e3f52c3c99f635825f9e63fd5abf3359731b05408fed215f19
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: display_glade_border.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2473, 2485
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2473,"record_offset":1604,"rva":29231824,"va":5397940944}
// 编译实例 RVAs: 0x1be0ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23128746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e856159ffe","size":5,"soft_fail":false,"target_va":5374813424,"va":5397942165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22859822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8d22fa3fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5397941305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22859595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8b530a3fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5397941094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be6110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8c0550000","size":5,"soft_fail":false,"target_va":5397963024,"va":5397941067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13905819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e89b2fd400","size":5,"soft_fail":false,"target_va":5411847168,"va":5397941344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13598972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8fc80cf00","size":5,"soft_fail":false,"target_va":5411540992,"va":5397942015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15101535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8a19119ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5397941738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23585986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e83e1b98fe","size":5,"soft_fail":false,"target_va":5374355568,"va":5397941549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13598968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8f880cf00","size":5,"soft_fail":false,"target_va":5411540963,"va":5397941990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20198856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e838cacbfe","size":5,"soft_fail":false,"target_va":5377743024,"va":5397941875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17514252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8f4c0f4fe","size":5,"soft_fail":false,"target_va":5380427664,"va":5397941911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10360942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e892e761ff","size":5,"soft_fail":false,"target_va":5387580080,"va":5397941017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10360657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8afe861ff","size":5,"soft_fail":false,"target_va":5387580992,"va":5397941644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17677816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e80842f2fe","size":5,"soft_fail":false,"target_va":5380263600,"va":5397941411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17677700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e87c42f2fe","size":5,"soft_fail":false,"target_va":5380263744,"va":5397941439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17677845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e8eb41f2fe","size":5,"soft_fail":false,"target_va":5380263536,"va":5397941376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::set_ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17677819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e80542f2fe","size":5,"soft_fail":false,"target_va":5380263648,"va":5397941462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17677694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397940944,"raw_hex":"e88242f2fe","size":5,"soft_fail":false,"target_va":5380263792,"va":5397941481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2473,"record_offset":4368,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2473,"record_offset":4480,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":3208,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2473,"record_offset":3016,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2473,"record_offset":2452,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":2473,"record_offset":3812,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":2473,"record_offset":4452,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2473,"record_offset":1872,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":1956,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2473,"record_offset":1788,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":1604,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":1604,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":1604,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":4544,"parent_record_offset":4512,"ancestor_record_offset":1604,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":3640,"parent_record_offset":3464,"ancestor_record_offset":1604,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":2140,"parent_record_offset":2052,"ancestor_record_offset":1604,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2473,"record_offset":4512,"parent_record_offset":4480,"ancestor_record_offset":1604,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2473,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":1604,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":1604,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":1604,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":2080,"parent_record_offset":2052,"ancestor_record_offset":1604,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1604,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":1604,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":1604,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":3464,"parent_record_offset":3432,"ancestor_record_offset":1604,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":3432,"parent_record_offset":3404,"ancestor_record_offset":1604,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":1604,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":3500,"parent_record_offset":3464,"ancestor_record_offset":1604,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":3340,"parent_record_offset":3308,"ancestor_record_offset":1604,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":3308,"parent_record_offset":3240,"ancestor_record_offset":1604,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":3976,"parent_record_offset":3936,"ancestor_record_offset":1604,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1604,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":3936,"parent_record_offset":3900,"ancestor_record_offset":1604,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":3376,"parent_record_offset":3340,"ancestor_record_offset":1604,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1604,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":2664,"parent_record_offset":2564,"ancestor_record_offset":1604,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2473,"record_offset":3872,"parent_record_offset":3836,"ancestor_record_offset":1604,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2473,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":1604,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2473,"record_offset":3900,"parent_record_offset":3836,"ancestor_record_offset":1604,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2473,"record_offset":2532,"parent_record_offset":2484,"ancestor_record_offset":1604,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2473,"record_offset":2564,"parent_record_offset":2532,"ancestor_record_offset":1604,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2473,"record_offset":3048,"parent_record_offset":3016,"ancestor_record_offset":1604,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":2473,"record_offset":2484,"parent_record_offset":2452,"ancestor_record_offset":1604,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2473,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":1604,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2473,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":1604,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2473,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":1604,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2473,"record_offset":3084,"parent_record_offset":3048,"ancestor_record_offset":1604,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":2473,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1604,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":1604,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1604,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":1604,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":2608,"parent_record_offset":2564,"ancestor_record_offset":1604,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":2108,"parent_record_offset":2080,"ancestor_record_offset":1604,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2473,"record_offset":3112,"parent_record_offset":3084,"ancestor_record_offset":1604,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2473,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":1604,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2473,"record_offset":3172,"parent_record_offset":3048,"ancestor_record_offset":1604,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2473,"record_offset":3740,"parent_record_offset":3340,"ancestor_record_offset":1604,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2473,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1604,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":1604,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":4088,"parent_record_offset":3936,"ancestor_record_offset":1604,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2473,"record_offset":2720,"parent_record_offset":2564,"ancestor_record_offset":1604,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2473,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1604,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2473,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":1604,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2473,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1604,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17D16
// 首条位置: {"module":2485,"inlinee":97558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2473,"record_offset":5688,"rva":29233280,"va":5397942400}
// 编译实例 RVAs: 0x1be1080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22860991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e8412ba3fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5397942474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22861656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e8a828a3fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5397943155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13904541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e89d2ad400","size":5,"soft_fail":false,"target_va":5411847168,"va":5397942622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13597890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e8c27ccf00","size":5,"soft_fail":false,"target_va":5411540992,"va":5397943097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13865324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e86c91d300","size":5,"soft_fail":false,"target_va":5411808352,"va":5397943023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13597878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e8b67ccf00","size":5,"soft_fail":false,"target_va":5411540963,"va":5397943080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20198682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e8e6cacbfe","size":5,"soft_fail":false,"target_va":5377744016,"va":5397942693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20198555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e865cbcbfe","size":5,"soft_fail":false,"target_va":5377744208,"va":5397942758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20199108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e83cc9cbfe","size":5,"soft_fail":false,"target_va":5377743616,"va":5397942719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20199386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e826c8cbfe","size":5,"soft_fail":false,"target_va":5377743280,"va":5397942661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20199114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397942400,"raw_hex":"e836c9cbfe","size":5,"soft_fail":false,"target_va":5377743568,"va":5397942677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":5864,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":7204,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":6012,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2473,"record_offset":6604,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":7288,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":5688,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":5688,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":5688,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":5896,"parent_record_offset":5864,"ancestor_record_offset":5688,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2473,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":5688,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2473,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":5688,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":5688,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":5688,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":5688,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6816,"parent_record_offset":6784,"ancestor_record_offset":5688,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":5688,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6256,"parent_record_offset":6224,"ancestor_record_offset":5688,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6224,"parent_record_offset":6196,"ancestor_record_offset":5688,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2473,"record_offset":6292,"parent_record_offset":6256,"ancestor_record_offset":5688,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":5688,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":5688,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2473,"record_offset":6104,"parent_record_offset":6040,"ancestor_record_offset":5688,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":6076,"parent_record_offset":6040,"ancestor_record_offset":5688,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":5688,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":6728,"parent_record_offset":6692,"ancestor_record_offset":5688,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":6168,"parent_record_offset":6132,"ancestor_record_offset":5688,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2473,"record_offset":6040,"parent_record_offset":6012,"ancestor_record_offset":5688,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2473,"record_offset":6660,"parent_record_offset":6632,"ancestor_record_offset":5688,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2473,"record_offset":6692,"parent_record_offset":6660,"ancestor_record_offset":5688,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2473,"record_offset":6632,"parent_record_offset":6604,"ancestor_record_offset":5688,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2473,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":5688,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2473,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":5688,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2473,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":5688,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":5688,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2473,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":5688,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2473,"record_offset":6532,"parent_record_offset":6132,"ancestor_record_offset":5688,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2473,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":5688,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02473.jsonl.gz"}
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
// 完整结构及原始类型记录: display_glade_border.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: display_glade_border.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: display_glade_border.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>", "bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border
// 信息 ID: 6f37cbc0db2ba72997f4c05f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a3d5662e39d62e11389000bd: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0xea266a -> 0x1be0ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > > @ 0x1be55bc -> 0x1be0ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$0
// 信息 ID: c5c3e97f45ec413d0f97d1b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; module=2485, inline_record=2544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$1
// 信息 ID: 060f79d255638d8eaaae2c3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ptr::drop_in_place<system_glade_border::display_glade_border::display_glade_border::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1be0a53 -> 0x1be1080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
