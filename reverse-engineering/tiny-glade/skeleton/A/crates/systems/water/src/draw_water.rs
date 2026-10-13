// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/water/src/draw_water.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fb1740f823786386771d39e1634b41aab0a6b3501b951d3e52c4846659bb95e3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_water.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2085, 3369
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_water::draw_water::draw_water
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2085,"record_offset":804,"rva":21011136,"va":5389720256}
// 编译实例 RVAs: 0x1409ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16971147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8750afdfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5389720838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16971304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8d809fdfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5389721011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16571586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e83e2303ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5389721421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597160
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15150246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e85ad318ff","size":5,"soft_fail":false,"target_va":5374570848,"va":5389721089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12780747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e835fb3cff","size":5,"soft_fail":false,"target_va":5376939904,"va":5389720646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a8e130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6832571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8bb416800","size":5,"soft_fail":false,"target_va":5396554032,"va":5389721456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13256663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8d747ca00","size":5,"soft_fail":false,"target_va":5402978144,"va":5389721476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12190911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e841fb45ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5389720794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-18415998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e882fee6fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5389721049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14628578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e81ec920ff","size":5,"soft_fail":false,"target_va":5375091920,"va":5389720493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_border::GladeBorder::is_shape_inside
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5295264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e86033afff","size":5,"soft_fail":false,"target_va":5384425296,"va":5389720555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_terrain_raycast::CameraCursorRay::intersect_with_xz_plane
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14157157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e89bfa27ff","size":5,"soft_fail":false,"target_va":5375563568,"va":5389720720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::DrawState::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5235327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8811db0ff","size":5,"soft_fail":false,"target_va":5384485856,"va":5389721178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14171867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e825c127ff","size":5,"soft_fail":false,"target_va":5375548608,"va":5389720470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14170730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e896c527ff","size":5,"soft_fail":false,"target_va":5375550352,"va":5389721077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8e4cd24ff","size":5,"soft_fail":false,"target_va":5375355744,"va":5389720951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7447360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e8c05c8eff","size":5,"soft_fail":false,"target_va":5382273632,"va":5389720987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11752685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389720256,"raw_hex":"e813ab4cff","size":5,"soft_fail":false,"target_va":5377967856,"va":5389720536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2085,"record_offset":1060,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2085,"record_offset":1144,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2085,"record_offset":940,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2085,"record_offset":1568,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":1368,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":804,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":804,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":804,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":804,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":804,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":1496,"parent_record_offset":1440,"ancestor_record_offset":804,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2085,"record_offset":1176,"parent_record_offset":1144,"ancestor_record_offset":804,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2085,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":804,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":1468,"parent_record_offset":1440,"ancestor_record_offset":804,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":804,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":804,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2085,"record_offset":1724,"parent_record_offset":1684,"ancestor_record_offset":804,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2085,"record_offset":1684,"parent_record_offset":1648,"ancestor_record_offset":804,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2085,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":804,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2085,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":804,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2085,"record_offset":1648,"parent_record_offset":1568,"ancestor_record_offset":804,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2085,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":804,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":804,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":1780,"parent_record_offset":1684,"ancestor_record_offset":804,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_water::draw_water::draw_water::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2085,"record_offset":2876,"rva":21012448,"va":5389721568}
// 编译实例 RVAs: 0x1409fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16971938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e85e07fdfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5389721629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16972301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e8f305fdfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5389722008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13256205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e80d46ca00","size":5,"soft_fail":false,"target_va":5402978144,"va":5389721934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e878ce24ff","size":5,"soft_fail":false,"target_va":5375356704,"va":5389721763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14364978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e8cece24ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5389721869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e8b8cb24ff","size":5,"soft_fail":false,"target_va":5375355984,"va":5389721747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e8c0cd24ff","size":5,"soft_fail":false,"target_va":5375356560,"va":5389721803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e80dce24ff","size":5,"soft_fail":false,"target_va":5375356656,"va":5389721822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14365571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721568,"raw_hex":"e87dcc24ff","size":5,"soft_fail":false,"target_va":5375356272,"va":5389721838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3020,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3724,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2085,"record_offset":3168,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":3808,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2876,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2876,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":2876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":2876,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2085,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":2876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2085,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":2876,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":2876,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":2876,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2085,"record_offset":3600,"parent_record_offset":3428,"ancestor_record_offset":2876,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":2876,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":2876,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":2876,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":2876,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":2876,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2085,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":2876,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2085,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2876,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2085,"record_offset":3196,"parent_record_offset":3168,"ancestor_record_offset":2876,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2085,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":2876,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2085,"record_offset":3400,"parent_record_offset":3364,"ancestor_record_offset":2876,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2085,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":2876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2085,"record_offset":3636,"parent_record_offset":3364,"ancestor_record_offset":2876,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2085,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2876,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_water::draw_water::draw_water::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F4B
// 首条位置: {"module":3369,"inlinee":106315}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":2280,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3369,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":2280,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":2356,"parent_record_offset":2328,"ancestor_record_offset":2280,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":2280,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":2388,"parent_record_offset":2304,"ancestor_record_offset":2280,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_water::draw_water::draw_water::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F4D
// 首条位置: {"module":3369,"inlinee":106317}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3369,"record_offset":3800,"parent_record_offset":3768,"ancestor_record_offset":3660,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":3736,"parent_record_offset":3708,"ancestor_record_offset":3660,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":3660,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":3768,"parent_record_offset":3684,"ancestor_record_offset":3660,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_water::draw_water::draw_water::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F4C
// 首条位置: {"module":3369,"inlinee":106316}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2784,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3369,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":2784,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3369,"record_offset":2972,"parent_record_offset":2932,"ancestor_record_offset":2784,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3369,"record_offset":2932,"parent_record_offset":2896,"ancestor_record_offset":2784,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":2868,"parent_record_offset":2832,"ancestor_record_offset":2784,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3369,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2784,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3369,"record_offset":2896,"parent_record_offset":2832,"ancestor_record_offset":2784,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3369,"record_offset":3028,"parent_record_offset":2932,"ancestor_record_offset":2784,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03369.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_water.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_water.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_water.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>", "bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>", "bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>

// ------------------------------------------------------------------
// 函数: system_water::draw_water::draw_water
// 信息 ID: f32f7fb6ccb6330a85d27043
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a6a99d18cb5018f6925c7dd8: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> @ 0x9845fb -> 0x1409ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::water::WaterEraseEvent>,bevy_ecs::event::EventWriter<system_water::record_water_history::RecordWaterEditToHistoryCmd>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> > @ 0x140c0ea -> 0x1409ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::draw_water::draw_water::closure$0
// 信息 ID: e196aff1dfcc71f254298f7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_water::draw_water::draw_water", "captures": "unknown"}
// [闭包环境类型引用] system_water::draw_water::draw_water::closure_env$0 <- core::ops::function::FnOnce::call_once<system_water::draw_water::draw_water::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_water::draw_water::draw_water::closure_env$0 <- core::ptr::drop_in_place<system_water::draw_water::draw_water::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_water::draw_water::draw_water::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1409ab2 -> 0x1409fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::draw_water::draw_water::closure$1
// 信息 ID: 14ec426603679c1eaa3f6847
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_water::draw_water::draw_water", "captures": "unknown"}
// [闭包环境类型引用] system_water::draw_water::draw_water::closure_env$1 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; module=3369, inline_record=2280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::draw_water::draw_water::closure$2
// 信息 ID: a0be5712cfc53294f9af8356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_water::draw_water::draw_water", "captures": "unknown"}
// [闭包环境类型引用] system_water::draw_water::draw_water::closure_env$2 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; module=3369, inline_record=3660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_water::draw_water::draw_water::closure$3
// 信息 ID: 36636f8d8399eb32bd259009
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_water::draw_water::draw_water", "captures": "unknown"}
// [闭包环境类型引用] system_water::draw_water::draw_water::closure_env$3 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>; module=3369, inline_record=2784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
