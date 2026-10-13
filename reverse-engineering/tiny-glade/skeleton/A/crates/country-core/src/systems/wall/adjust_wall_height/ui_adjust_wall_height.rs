// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d281c90c2b0e9f3c6c61c139484ef47f1c934c8bfc97bacb3f5c4d13dc034b31
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_adjust_wall_height.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 453
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":12848,"rva":6409440,"va":5375118560}
// 编译实例 RVAs: 0x61cce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: ui_adjust_wall_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5506405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e89bfaabff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375119920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_adjust_wall_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2371452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e884d0dbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375121159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_adjust_wall_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7738643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e813157600","size":5,"soft_fail":false,"target_va":5382858112,"va":5375119464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_adjust_wall_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8189127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8c7f47c00","size":5,"soft_fail":false,"target_va":5383308144,"va":5375119012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3815964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8e4c5c5ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5375120839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27857358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8ce11a901","size":5,"soft_fail":false,"target_va":5402978688,"va":5375121325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27857651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8f312a901","size":5,"soft_fail":false,"target_va":5402978880,"va":5375121224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27857897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8e913a901","size":5,"soft_fail":false,"target_va":5402979152,"va":5375121250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8c9e3ffff","size":5,"soft_fail":false,"target_va":5375113952,"va":5375121170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27861059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e84320a901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375119064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e85feeffff","size":5,"soft_fail":false,"target_va":5375114144,"va":5375118652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27861922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8a223a901","size":5,"soft_fail":false,"target_va":5402980656,"va":5375118729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7356525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e86d407000","size":5,"soft_fail":false,"target_va":5382477248,"va":5375120718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7355478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8563c7000","size":5,"soft_fail":false,"target_va":5382476112,"va":5375120629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1237677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8ade21200","size":5,"soft_fail":false,"target_va":5376356912,"va":5375119230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1241563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e8dbf11200","size":5,"soft_fail":false,"target_va":5376360560,"va":5375118992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1237506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375118560,"raw_hex":"e802e21200","size":5,"soft_fail":false,"target_va":5376356144,"va":5375118633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":16760,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":17128,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":20468,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":15744,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":16864,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":16048,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":453,"record_offset":18364,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":20356,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":20320,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":21388,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":453,"record_offset":21680,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_animation::RoofAnimation::is_being_animated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_animation.rs
//     evidence={"module":453,"record_offset":19224,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":25993,"source_line":12,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":453,"record_offset":20412,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":19252,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":17276,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":23583,"source_line":67,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":453,"record_offset":16240,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":453,"record_offset":20524,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":22964,"source_line":238,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":453,"record_offset":20440,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":25995,"source_line":32,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":14696,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":13108,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":13684,"parent_record_offset":12848,"ancestor_record_offset":12848,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":21624,"parent_record_offset":21600,"ancestor_record_offset":12848,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":453,"record_offset":21600,"parent_record_offset":21508,"ancestor_record_offset":12848,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":21536,"parent_record_offset":21508,"ancestor_record_offset":12848,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":17000,"parent_record_offset":16968,"ancestor_record_offset":12848,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":12848,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":16968,"parent_record_offset":16928,"ancestor_record_offset":12848,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":16928,"parent_record_offset":16892,"ancestor_record_offset":12848,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":16724,"parent_record_offset":16700,"ancestor_record_offset":12848,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":21412,"parent_record_offset":21388,"ancestor_record_offset":12848,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":453,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":12848,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":453,"record_offset":18572,"parent_record_offset":18428,"ancestor_record_offset":12848,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":12848,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":17068,"parent_record_offset":16864,"ancestor_record_offset":12848,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":453,"record_offset":18428,"parent_record_offset":18388,"ancestor_record_offset":12848,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":15868,"parent_record_offset":15832,"ancestor_record_offset":12848,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":12848,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":15832,"parent_record_offset":15780,"ancestor_record_offset":12848,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":16080,"parent_record_offset":16048,"ancestor_record_offset":12848,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":453,"record_offset":18848,"parent_record_offset":18388,"ancestor_record_offset":12848,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":453,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":12848,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":453,"record_offset":18388,"parent_record_offset":18364,"ancestor_record_offset":12848,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":453,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":12848,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":12848,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":18924,"parent_record_offset":18900,"ancestor_record_offset":12848,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":12848,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":453,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":12848,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":453,"record_offset":19152,"parent_record_offset":18872,"ancestor_record_offset":12848,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":453,"record_offset":18900,"parent_record_offset":18872,"ancestor_record_offset":12848,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":453,"record_offset":18820,"parent_record_offset":18388,"ancestor_record_offset":12848,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":453,"record_offset":15780,"parent_record_offset":15744,"ancestor_record_offset":12848,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":453,"record_offset":18636,"parent_record_offset":18604,"ancestor_record_offset":12848,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15096,"parent_record_offset":15068,"ancestor_record_offset":12848,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":12848,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15124,"parent_record_offset":15096,"ancestor_record_offset":12848,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15036,"parent_record_offset":15008,"ancestor_record_offset":12848,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15068,"parent_record_offset":15036,"ancestor_record_offset":12848,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15240,"parent_record_offset":15068,"ancestor_record_offset":12848,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":453,"record_offset":15352,"parent_record_offset":15328,"ancestor_record_offset":12848,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":12848,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":453,"record_offset":18004,"parent_record_offset":17980,"ancestor_record_offset":12848,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":16552,"parent_record_offset":16524,"ancestor_record_offset":12848,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":17556,"parent_record_offset":17532,"ancestor_record_offset":12848,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":453,"record_offset":16612,"parent_record_offset":16580,"ancestor_record_offset":12848,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":453,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":12848,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":453,"record_offset":16112,"parent_record_offset":16080,"ancestor_record_offset":12848,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":453,"record_offset":16700,"parent_record_offset":16240,"ancestor_record_offset":12848,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":453,"record_offset":16796,"parent_record_offset":16760,"ancestor_record_offset":12848,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":453,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":12848,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":15912,"parent_record_offset":15868,"ancestor_record_offset":12848,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":453,"record_offset":21564,"parent_record_offset":21536,"ancestor_record_offset":12848,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":453,"record_offset":13320,"parent_record_offset":13224,"ancestor_record_offset":12848,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":12848,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":453,"record_offset":18516,"parent_record_offset":18428,"ancestor_record_offset":12848,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":16144,"parent_record_offset":16112,"ancestor_record_offset":12848,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":14932,"parent_record_offset":14904,"ancestor_record_offset":12848,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":18332,"parent_record_offset":17276,"ancestor_record_offset":12848,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":14280,"parent_record_offset":13820,"ancestor_record_offset":12848,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":14980,"parent_record_offset":14804,"ancestor_record_offset":12848,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":15976,"parent_record_offset":15868,"ancestor_record_offset":12848,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":16204,"parent_record_offset":16080,"ancestor_record_offset":12848,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":453,"record_offset":13632,"parent_record_offset":13604,"ancestor_record_offset":12848,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":15304,"parent_record_offset":15008,"ancestor_record_offset":12848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":16832,"parent_record_offset":16760,"ancestor_record_offset":12848,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":12848,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":453,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":12848,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":17244,"parent_record_offset":17128,"ancestor_record_offset":12848,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":12848,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":17188,"parent_record_offset":17128,"ancestor_record_offset":12848,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":12848,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":453,"record_offset":14240,"parent_record_offset":14208,"ancestor_record_offset":12848,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":12848,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":15196,"parent_record_offset":15172,"ancestor_record_offset":12848,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":453,"record_offset":15704,"parent_record_offset":14768,"ancestor_record_offset":12848,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":453,"record_offset":18792,"parent_record_offset":18388,"ancestor_record_offset":12848,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":13256,"parent_record_offset":13224,"ancestor_record_offset":12848,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":453,"record_offset":13284,"parent_record_offset":13256,"ancestor_record_offset":12848,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":12848,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":12848,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":453,"record_offset":13984,"parent_record_offset":13876,"ancestor_record_offset":12848,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":13224,"parent_record_offset":13196,"ancestor_record_offset":12848,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":453,"record_offset":13716,"parent_record_offset":13684,"ancestor_record_offset":12848,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":13848,"parent_record_offset":13820,"ancestor_record_offset":12848,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":453,"record_offset":13820,"parent_record_offset":13788,"ancestor_record_offset":12848,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":12848,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":12848,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":12848,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":453,"record_offset":14312,"parent_record_offset":13788,"ancestor_record_offset":12848,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13420,"parent_record_offset":13388,"ancestor_record_offset":12848,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13580,"parent_record_offset":13420,"ancestor_record_offset":12848,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":12848,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13140,"parent_record_offset":13108,"ancestor_record_offset":12848,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13604,"parent_record_offset":13580,"ancestor_record_offset":12848,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12848,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":453,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":12848,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":13876,"parent_record_offset":13848,"ancestor_record_offset":12848,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":14124,"parent_record_offset":13876,"ancestor_record_offset":12848,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":14096,"parent_record_offset":13876,"ancestor_record_offset":12848,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":453,"record_offset":13928,"parent_record_offset":13876,"ancestor_record_offset":12848,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":453,"record_offset":18028,"parent_record_offset":18004,"ancestor_record_offset":12848,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":453,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":12848,"inlinee":22965,"source_line":134,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":18604,"parent_record_offset":18388,"ancestor_record_offset":12848,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":12848,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":12848,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":453,"record_offset":18664,"parent_record_offset":18636,"ancestor_record_offset":12848,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":453,"record_offset":17980,"parent_record_offset":17956,"ancestor_record_offset":12848,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":12848,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":17348,"parent_record_offset":17324,"ancestor_record_offset":12848,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":16316,"parent_record_offset":16288,"ancestor_record_offset":12848,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":17956,"parent_record_offset":17820,"ancestor_record_offset":12848,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":453,"record_offset":17380,"parent_record_offset":17348,"ancestor_record_offset":12848,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":12848,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17844,"parent_record_offset":17820,"ancestor_record_offset":12848,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":12848,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":18164,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16464,"parent_record_offset":16372,"ancestor_record_offset":12848,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":18200,"parent_record_offset":17432,"ancestor_record_offset":12848,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":12848,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17408,"parent_record_offset":17348,"ancestor_record_offset":12848,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16344,"parent_record_offset":16316,"ancestor_record_offset":12848,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16436,"parent_record_offset":16372,"ancestor_record_offset":12848,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16408,"parent_record_offset":16372,"ancestor_record_offset":12848,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17460,"parent_record_offset":17432,"ancestor_record_offset":12848,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":16372,"parent_record_offset":16344,"ancestor_record_offset":12848,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":17700,"parent_record_offset":17672,"ancestor_record_offset":12848,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":17728,"parent_record_offset":17700,"ancestor_record_offset":12848,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":17788,"parent_record_offset":17672,"ancestor_record_offset":12848,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":453,"record_offset":17672,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17504,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":453,"record_offset":17820,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":17532,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":16524,"parent_record_offset":16464,"ancestor_record_offset":12848,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":17616,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":18072,"parent_record_offset":17460,"ancestor_record_offset":12848,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":453,"record_offset":16580,"parent_record_offset":16524,"ancestor_record_offset":12848,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":453,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":12848,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":14732,"parent_record_offset":14696,"ancestor_record_offset":12848,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":453,"record_offset":14844,"parent_record_offset":14804,"ancestor_record_offset":12848,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":15008,"parent_record_offset":14804,"ancestor_record_offset":12848,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":453,"record_offset":15276,"parent_record_offset":15008,"ancestor_record_offset":12848,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":12848,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":16288,"parent_record_offset":16264,"ancestor_record_offset":12848,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":453,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":12848,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":453,"record_offset":14872,"parent_record_offset":14844,"ancestor_record_offset":12848,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":453,"record_offset":14904,"parent_record_offset":14872,"ancestor_record_offset":12848,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":14804,"parent_record_offset":14768,"ancestor_record_offset":12848,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":453,"record_offset":14768,"parent_record_offset":14732,"ancestor_record_offset":12848,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":20288,"parent_record_offset":19252,"ancestor_record_offset":12848,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00453.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_adjust_wall_height.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_adjust_wall_height.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_adjust_wall_height.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height
// 信息 ID: e2948ab150cc082bb52c9852
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3ef73f5553627697ddcc035f: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xb097e -> 0x61cce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_animation::RoofAnimation>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x311f39 -> 0x61cce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
