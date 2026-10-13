// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/ui_lintel_controls.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f5967476530468228aad43bb08472aad793f76733ba34e3efe168196be4cd3e9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_lintel_controls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 375
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":375,"record_offset":4632,"rva":5204128,"va":5373913248}
// 编译实例 RVAs: 0x4f68a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1167722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8962eeeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373917413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1166349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8f333eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373916056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=2
//     target_aliases=18 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t478479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e80f4d0700","size":5,"soft_fail":false,"target_va":5374395248,"va":5373916764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97f60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7995276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e88cff7900","size":5,"soft_fail":false,"target_va":5381914464,"va":5373919183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cede10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8351485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8fd6e7f00","size":5,"soft_fail":false,"target_va":5382266384,"va":5373914894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cee5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8352626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e872737f00","size":5,"soft_fail":false,"target_va":5382268400,"va":5373915769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29675898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e87ad1c401","size":5,"soft_fail":false,"target_va":5403593360,"va":5373917457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29059484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e89c69bb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373918655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3614245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e825263700","size":5,"soft_fail":false,"target_va":5377529888,"va":5373915638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29136188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e83c95bc01","size":5,"soft_fail":false,"target_va":5403053728,"va":5373917535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2606756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e85c39d8ff","size":5,"soft_fail":false,"target_va":5371308608,"va":5373915359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3608443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e87b0f3700","size":5,"soft_fail":false,"target_va":5377527120,"va":5373918672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29059840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8006bbb01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373918843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t29060154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e83a6cbb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373918721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29060718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e86e6ebb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373918621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8a0f7ffff","size":5,"soft_fail":false,"target_va":5373911920,"va":5373914059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29066294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e83684bb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373913829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8fffaffff","size":5,"soft_fail":false,"target_va":5373912112,"va":5373913388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29067186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8b287bb01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373913465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1049449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e869031000","size":5,"soft_fail":false,"target_va":5374966368,"va":5373916914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t902998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e856c70d00","size":5,"soft_fail":false,"target_va":5374819872,"va":5373916869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t902522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e87ac50d00","size":5,"soft_fail":false,"target_va":5374819344,"va":5373916817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1369637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e825e61400","size":5,"soft_fail":false,"target_va":5375285936,"va":5373916294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1371195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e83bec1400","size":5,"soft_fail":false,"target_va":5375287584,"va":5373916384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1370866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8f2ea1400","size":5,"soft_fail":false,"target_va":5375287616,"va":5373916745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1520372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8f4321700","size":5,"soft_fail":false,"target_va":5375434976,"va":5373914599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1634554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8faf01800","size":5,"soft_fail":false,"target_va":5375550352,"va":5373915793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1634529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8e1f01800","size":5,"soft_fail":false,"target_va":5375550384,"va":5373915850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1633193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8a9eb1800","size":5,"soft_fail":false,"target_va":5375550448,"va":5373917250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1855420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8bc4f1c00","size":5,"soft_fail":false,"target_va":5375771168,"va":5373915743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1437654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8d6ef1500","size":5,"soft_fail":false,"target_va":5375355744,"va":5373918085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8355493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8a57e7f00","size":5,"soft_fail":false,"target_va":5382273632,"va":5373918134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t971696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8b0d30e00","size":5,"soft_fail":false,"target_va":5374887792,"va":5373916091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t971712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8c0d30e00","size":5,"soft_fail":false,"target_va":5374887840,"va":5373916123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t971670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e896d30e00","size":5,"soft_fail":false,"target_va":5374887920,"va":5373916245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2442912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8a0462500","size":5,"soft_fail":false,"target_va":5376356912,"va":5373913995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2446795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e8cb552500","size":5,"soft_fail":false,"target_va":5376360560,"va":5373913760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2442771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e813462500","size":5,"soft_fail":false,"target_va":5376356144,"va":5373913368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7951639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e817557900","size":5,"soft_fail":false,"target_va":5381865424,"va":5373913780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8351599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913248,"raw_hex":"e86f6f7f00","size":5,"soft_fail":false,"target_va":5382266928,"va":5373915324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":375,"record_offset":10072,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":375,"record_offset":13312,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":9988,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":8548,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":9272,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":11380,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":12996,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":8576,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":11956,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":375,"record_offset":10016,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":375,"record_offset":10600,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":375,"record_offset":10512,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":375,"record_offset":11288,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":375,"record_offset":8892,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":375,"record_offset":10448,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":8492,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":10156,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":10228,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":375,"record_offset":11204,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":8688,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":9612,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":9556,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":375,"record_offset":9088,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":375,"record_offset":9412,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":6496,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":4896,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":5484,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":375,"record_offset":7564,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":375,"record_offset":7896,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":375,"record_offset":10900,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":4632,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":4632,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":10404,"parent_record_offset":10380,"ancestor_record_offset":4632,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":13384,"parent_record_offset":13348,"ancestor_record_offset":4632,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":11668,"parent_record_offset":11568,"ancestor_record_offset":4632,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":10380,"parent_record_offset":10280,"ancestor_record_offset":4632,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":375,"record_offset":13348,"parent_record_offset":13312,"ancestor_record_offset":4632,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":375,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":4632,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":10312,"parent_record_offset":10280,"ancestor_record_offset":4632,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":12112,"parent_record_offset":12084,"ancestor_record_offset":4632,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":4632,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":12084,"parent_record_offset":12060,"ancestor_record_offset":4632,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":4632,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":4632,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":11788,"parent_record_offset":11540,"ancestor_record_offset":4632,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":4632,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":11476,"parent_record_offset":11444,"ancestor_record_offset":4632,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":11444,"parent_record_offset":11412,"ancestor_record_offset":4632,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":11412,"parent_record_offset":11380,"ancestor_record_offset":4632,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":9316,"parent_record_offset":9272,"ancestor_record_offset":4632,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":4632,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":10716,"parent_record_offset":10680,"ancestor_record_offset":4632,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":12060,"parent_record_offset":12020,"ancestor_record_offset":4632,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":4632,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":4632,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":12756,"parent_record_offset":12728,"ancestor_record_offset":4632,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":375,"record_offset":12204,"parent_record_offset":12020,"ancestor_record_offset":4632,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":375,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":4632,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":375,"record_offset":11988,"parent_record_offset":11956,"ancestor_record_offset":4632,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":375,"record_offset":10040,"parent_record_offset":10016,"ancestor_record_offset":4632,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":375,"record_offset":10624,"parent_record_offset":10600,"ancestor_record_offset":4632,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":375,"record_offset":10680,"parent_record_offset":10600,"ancestor_record_offset":4632,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":375,"record_offset":11604,"parent_record_offset":11568,"ancestor_record_offset":4632,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":375,"record_offset":11632,"parent_record_offset":11604,"ancestor_record_offset":4632,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":4632,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":4632,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":4632,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":6856,"parent_record_offset":6824,"ancestor_record_offset":4632,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":6888,"parent_record_offset":6856,"ancestor_record_offset":4632,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":7060,"parent_record_offset":6888,"ancestor_record_offset":4632,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":375,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":4632,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":375,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":4632,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":375,"record_offset":7812,"parent_record_offset":7780,"ancestor_record_offset":4632,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":375,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":4632,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":375,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":4632,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$3::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":375,"record_offset":12296,"parent_record_offset":12264,"ancestor_record_offset":4632,"inlinee":21623,"source_line":162,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":375,"record_offset":12468,"parent_record_offset":12328,"ancestor_record_offset":4632,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":375,"record_offset":12264,"parent_record_offset":12232,"ancestor_record_offset":4632,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":9028,"parent_record_offset":8892,"ancestor_record_offset":4632,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":9148,"parent_record_offset":9112,"ancestor_record_offset":4632,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":12380,"parent_record_offset":12328,"ancestor_record_offset":4632,"inlinee":21626,"source_line":124,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":4632,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":4632,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":375,"record_offset":12420,"parent_record_offset":12380,"ancestor_record_offset":4632,"inlinee":21627,"source_line":368,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":375,"record_offset":12328,"parent_record_offset":12296,"ancestor_record_offset":4632,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":375,"record_offset":9208,"parent_record_offset":9112,"ancestor_record_offset":4632,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":375,"record_offset":12232,"parent_record_offset":12020,"ancestor_record_offset":4632,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":375,"record_offset":12728,"parent_record_offset":12468,"ancestor_record_offset":4632,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":375,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":4632,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":375,"record_offset":10344,"parent_record_offset":10312,"ancestor_record_offset":4632,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":375,"record_offset":5120,"parent_record_offset":5024,"ancestor_record_offset":4632,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":375,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":4632,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":375,"record_offset":9580,"parent_record_offset":9556,"ancestor_record_offset":4632,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":375,"record_offset":9644,"parent_record_offset":9612,"ancestor_record_offset":4632,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4632,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":4632,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":6080,"parent_record_offset":5620,"ancestor_record_offset":4632,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":4632,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":375,"record_offset":6796,"parent_record_offset":6616,"ancestor_record_offset":4632,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":375,"record_offset":5432,"parent_record_offset":5404,"ancestor_record_offset":4632,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":375,"record_offset":11828,"parent_record_offset":11476,"ancestor_record_offset":4632,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":7124,"parent_record_offset":6824,"ancestor_record_offset":4632,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":375,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":4632,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":375,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":4632,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":375,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":4632,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":375,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":4632,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":375,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":4632,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":375,"record_offset":6040,"parent_record_offset":6008,"ancestor_record_offset":4632,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":11744,"parent_record_offset":11716,"ancestor_record_offset":4632,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":4632,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":375,"record_offset":7524,"parent_record_offset":6576,"ancestor_record_offset":4632,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":375,"record_offset":5056,"parent_record_offset":5024,"ancestor_record_offset":4632,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":375,"record_offset":8600,"parent_record_offset":8576,"ancestor_record_offset":4632,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":375,"record_offset":13024,"parent_record_offset":12996,"ancestor_record_offset":4632,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":375,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":4632,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":375,"record_offset":13052,"parent_record_offset":13024,"ancestor_record_offset":4632,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":375,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":4632,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":4632,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":4632,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4632,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":4632,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":375,"record_offset":5784,"parent_record_offset":5676,"ancestor_record_offset":4632,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":375,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":4632,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":375,"record_offset":5516,"parent_record_offset":5484,"ancestor_record_offset":4632,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":375,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":4632,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":375,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":4632,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":375,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":4632,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":375,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":4632,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":375,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":4632,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":375,"record_offset":6112,"parent_record_offset":5588,"ancestor_record_offset":4632,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":5220,"parent_record_offset":5188,"ancestor_record_offset":4632,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":5380,"parent_record_offset":5220,"ancestor_record_offset":4632,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4632,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":4632,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":4632,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":4632,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":375,"record_offset":4960,"parent_record_offset":4932,"ancestor_record_offset":4632,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":375,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":4632,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":375,"record_offset":5924,"parent_record_offset":5676,"ancestor_record_offset":4632,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":375,"record_offset":5896,"parent_record_offset":5676,"ancestor_record_offset":4632,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":375,"record_offset":5728,"parent_record_offset":5676,"ancestor_record_offset":4632,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":12588,"parent_record_offset":12496,"ancestor_record_offset":4632,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":4632,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":375,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":4632,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":375,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":4632,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":8280,"parent_record_offset":7972,"ancestor_record_offset":4632,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7696,"parent_record_offset":7636,"ancestor_record_offset":4632,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":4632,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":4632,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7668,"parent_record_offset":7636,"ancestor_record_offset":4632,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":4632,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":375,"record_offset":8052,"parent_record_offset":8020,"ancestor_record_offset":4632,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":375,"record_offset":8084,"parent_record_offset":8052,"ancestor_record_offset":4632,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":375,"record_offset":8020,"parent_record_offset":7972,"ancestor_record_offset":4632,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":375,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":4632,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":375,"record_offset":7724,"parent_record_offset":7696,"ancestor_record_offset":4632,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":375,"record_offset":7780,"parent_record_offset":7724,"ancestor_record_offset":4632,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":375,"record_offset":9112,"parent_record_offset":9088,"ancestor_record_offset":4632,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":6536,"parent_record_offset":6496,"ancestor_record_offset":4632,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":375,"record_offset":6660,"parent_record_offset":6616,"ancestor_record_offset":4632,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":6824,"parent_record_offset":6616,"ancestor_record_offset":4632,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":375,"record_offset":7096,"parent_record_offset":6824,"ancestor_record_offset":4632,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":375,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":4632,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":375,"record_offset":6720,"parent_record_offset":6688,"ancestor_record_offset":4632,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":375,"record_offset":6616,"parent_record_offset":6576,"ancestor_record_offset":4632,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":375,"record_offset":6576,"parent_record_offset":6536,"ancestor_record_offset":4632,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/ui_lintel_controls.rs
//     evidence={"module":375,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":4632,"inlinee":21629,"source_line":183,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x547D
// 首条位置: {"module":375,"inlinee":21629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":12588,"parent_record_offset":12496,"ancestor_record_offset":12496,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":375,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":12496,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":375,"record_offset":12552,"parent_record_offset":12524,"ancestor_record_offset":12496,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":375,"record_offset":12628,"parent_record_offset":12588,"ancestor_record_offset":12496,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":375,"record_offset":14884,"rva":5212192,"va":5373921312}
// 编译实例 RVAs: 0x4f8820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1171688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e8181feeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373921379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1172163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e83d1deeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373921870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29671859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e8b3c1c401","size":5,"soft_fail":false,"target_va":5403593360,"va":5373921496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29056349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e85d5dbb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373921790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28997439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e83f77ba01","size":5,"soft_fail":false,"target_va":5402919072,"va":5373921628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3605320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e848033700","size":5,"soft_fail":false,"target_va":5377527120,"va":5373921795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3324880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e8d0bb3200","size":5,"soft_fail":false,"target_va":5377246528,"va":5373921643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1435148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e80ce61500","size":5,"soft_fail":false,"target_va":5375356704,"va":5373921551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1435309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e8ade61500","size":5,"soft_fail":false,"target_va":5375356896,"va":5373921582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1434444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373921312,"raw_hex":"e84ce31500","size":5,"soft_fail":false,"target_va":5375355984,"va":5373921535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":15060,"parent_record_offset":14884,"ancestor_record_offset":14884,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":16264,"parent_record_offset":14884,"ancestor_record_offset":14884,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":15208,"parent_record_offset":14884,"ancestor_record_offset":14884,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":375,"record_offset":15708,"parent_record_offset":14884,"ancestor_record_offset":14884,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":16348,"parent_record_offset":14884,"ancestor_record_offset":14884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14884,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":14884,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":14884,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":15092,"parent_record_offset":15060,"ancestor_record_offset":14884,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":375,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":14884,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":375,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":14884,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15936,"parent_record_offset":15908,"ancestor_record_offset":14884,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15908,"parent_record_offset":15876,"ancestor_record_offset":14884,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15876,"parent_record_offset":15848,"ancestor_record_offset":14884,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15384,"parent_record_offset":15356,"ancestor_record_offset":14884,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15356,"parent_record_offset":15328,"ancestor_record_offset":14884,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":375,"record_offset":15596,"parent_record_offset":15356,"ancestor_record_offset":14884,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":15292,"parent_record_offset":15264,"ancestor_record_offset":14884,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":16288,"parent_record_offset":16264,"ancestor_record_offset":14884,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":15264,"parent_record_offset":15236,"ancestor_record_offset":14884,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":375,"record_offset":15236,"parent_record_offset":15208,"ancestor_record_offset":14884,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":16312,"parent_record_offset":16288,"ancestor_record_offset":14884,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":15988,"parent_record_offset":15820,"ancestor_record_offset":14884,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":15848,"parent_record_offset":15820,"ancestor_record_offset":14884,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":375,"record_offset":15328,"parent_record_offset":15292,"ancestor_record_offset":14884,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":375,"record_offset":15756,"parent_record_offset":15732,"ancestor_record_offset":14884,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":375,"record_offset":15784,"parent_record_offset":15756,"ancestor_record_offset":14884,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":375,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":14884,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":375,"record_offset":16020,"parent_record_offset":15784,"ancestor_record_offset":14884,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":375,"record_offset":15820,"parent_record_offset":15784,"ancestor_record_offset":14884,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":375,"record_offset":15420,"parent_record_offset":15384,"ancestor_record_offset":14884,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":375,"record_offset":15444,"parent_record_offset":15420,"ancestor_record_offset":14884,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":375,"record_offset":15660,"parent_record_offset":15636,"ancestor_record_offset":14884,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":375,"record_offset":15636,"parent_record_offset":15292,"ancestor_record_offset":14884,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":375,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":14884,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00375.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_lintel_controls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_lintel_controls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_lintel_controls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls
// 信息 ID: 299baf127a006ea67e3fca84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c5261df0a122ff1a8af7939b: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)> > @ 0xae4b0 -> 0x4f68a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)> @ 0x39e186 -> 0x4f68a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
// 信息 ID: 20c7f09d33cdd65e5e2945df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::ui_lintel_controls::ui_lintel_controls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::ui_lintel_controls::ui_lintel_controls; module=375, inline_record=12496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$1
// 信息 ID: 62b761dad2511886932eb2b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::ui_lintel_controls::ui_lintel_controls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1 <- core::ops::function::FnOnce::call_once<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1 <- core::ptr::drop_in_place<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_wall_constructor::ui_arch_controls::ui_arch_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x4f62d3 -> 0x4f8820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
