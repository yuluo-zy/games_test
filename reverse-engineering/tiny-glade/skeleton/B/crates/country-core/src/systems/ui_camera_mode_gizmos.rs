// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ui_camera_mode_gizmos.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: eee3127e36d65def402801e998262c359c76af2dbfe5c6557ef19291b297553f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_camera_mode_gizmos.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 513
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":32496,"rva":9775008,"va":5378484128}
// 编译实例 RVAs: 0x9527a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3403125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e88b12ccff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378484608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3403299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e8dd11ccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378484798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3158377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e897cecfff","size":5,"soft_fail":false,"target_va":5375326672,"va":5378485044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409236d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-192981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e82b0efdff","size":5,"soft_fail":false,"target_va":5378291408,"va":5378484384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11640404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e8549eb100","size":5,"soft_fail":false,"target_va":5390125184,"va":5378484775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33362420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e8f411fd01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378484743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33056140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e88c65f801","size":5,"soft_fail":false,"target_va":5411540992,"va":5378484847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4355633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e831764200","size":5,"soft_fail":false,"target_va":5382840208,"va":5378484570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33056084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378484128,"raw_hex":"e85465f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378484874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":32940,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":33024,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":513,"record_offset":32884,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":34112,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":513,"record_offset":32772,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":27178,"source_line":240,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::EscMenuState::is_open
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":513,"record_offset":32688,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":29297,"source_line":145,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":513,"record_offset":32744,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":27049,"source_line":432,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::main_camera::CameraRotationPivot::use_camera_cursor_grab_icon
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/main_camera.rs
//     evidence={"module":513,"record_offset":32912,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":28608,"source_line":234,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":513,"record_offset":32828,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":28896,"source_line":77,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":33240,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":513,"record_offset":34084,"parent_record_offset":32496,"ancestor_record_offset":32496,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":32496,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":33160,"parent_record_offset":33136,"ancestor_record_offset":32496,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34288,"parent_record_offset":34264,"ancestor_record_offset":32496,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":33080,"parent_record_offset":33052,"ancestor_record_offset":32496,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":33112,"parent_record_offset":33080,"ancestor_record_offset":32496,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34264,"parent_record_offset":34208,"ancestor_record_offset":32496,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":513,"record_offset":33052,"parent_record_offset":33024,"ancestor_record_offset":32496,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":513,"record_offset":32988,"parent_record_offset":32964,"ancestor_record_offset":32496,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":33508,"parent_record_offset":33480,"ancestor_record_offset":32496,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":34236,"parent_record_offset":34208,"ancestor_record_offset":32496,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":34208,"parent_record_offset":34184,"ancestor_record_offset":32496,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":33568,"parent_record_offset":33536,"ancestor_record_offset":32496,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":33536,"parent_record_offset":33508,"ancestor_record_offset":32496,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":34184,"parent_record_offset":34160,"ancestor_record_offset":32496,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":33604,"parent_record_offset":33568,"ancestor_record_offset":32496,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":33444,"parent_record_offset":33416,"ancestor_record_offset":32496,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":33416,"parent_record_offset":33388,"ancestor_record_offset":32496,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":33360,"parent_record_offset":33332,"ancestor_record_offset":32496,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":33388,"parent_record_offset":33360,"ancestor_record_offset":32496,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":33332,"parent_record_offset":33304,"ancestor_record_offset":32496,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":33304,"parent_record_offset":33276,"ancestor_record_offset":32496,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":33276,"parent_record_offset":33240,"ancestor_record_offset":32496,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":33952,"parent_record_offset":33928,"ancestor_record_offset":32496,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":33480,"parent_record_offset":33444,"ancestor_record_offset":32496,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":33628,"parent_record_offset":33604,"ancestor_record_offset":32496,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":33652,"parent_record_offset":33628,"ancestor_record_offset":32496,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":33868,"parent_record_offset":33844,"ancestor_record_offset":32496,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":32496,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":33676,"parent_record_offset":33652,"ancestor_record_offset":32496,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":513,"record_offset":32964,"parent_record_offset":32940,"ancestor_record_offset":32496,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":33844,"parent_record_offset":33444,"ancestor_record_offset":32496,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":33928,"parent_record_offset":33240,"ancestor_record_offset":32496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":33188,"parent_record_offset":33160,"ancestor_record_offset":32496,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":33988,"parent_record_offset":33952,"ancestor_record_offset":32496,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":34012,"parent_record_offset":33988,"ancestor_record_offset":32496,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":34044,"parent_record_offset":33952,"ancestor_record_offset":32496,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::EscMenuState::is_closed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":513,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":32496,"inlinee":27478,"source_line":149,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":34572,"rva":9775968,"va":5378485088}
// 编译实例 RVAs: 0x952b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3403685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e85b10ccff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378485168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3404330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e8d60dccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378485829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-753743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e8b17ff4ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5378485738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33055349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e87562f801","size":5,"soft_fail":false,"target_va":5411540992,"va":5378485638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33055298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e84262f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378485660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10009225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e889ba9800","size":5,"soft_fail":false,"target_va":5388494784,"va":5378485554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-741208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378485088,"raw_hex":"e8a8b0f4ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5378485411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34748,"parent_record_offset":34572,"ancestor_record_offset":34572,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":36304,"parent_record_offset":34572,"ancestor_record_offset":34572,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":36388,"parent_record_offset":34572,"ancestor_record_offset":34572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_camera_mode_gizmos::cursor_icon
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_camera_mode_gizmos.rs
//     evidence={"module":513,"record_offset":34896,"parent_record_offset":34572,"ancestor_record_offset":34572,"inlinee":29443,"source_line":89,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34804,"parent_record_offset":34780,"ancestor_record_offset":34572,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34828,"parent_record_offset":34804,"ancestor_record_offset":34572,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":36460,"parent_record_offset":36436,"ancestor_record_offset":34572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":34780,"parent_record_offset":34748,"ancestor_record_offset":34572,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":36436,"parent_record_offset":36412,"ancestor_record_offset":34572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":513,"record_offset":36412,"parent_record_offset":36388,"ancestor_record_offset":34572,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35244,"parent_record_offset":35216,"ancestor_record_offset":34572,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35304,"parent_record_offset":35272,"ancestor_record_offset":34572,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35272,"parent_record_offset":35244,"ancestor_record_offset":34572,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35180,"parent_record_offset":35152,"ancestor_record_offset":34572,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":36328,"parent_record_offset":36304,"ancestor_record_offset":34572,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35152,"parent_record_offset":35124,"ancestor_record_offset":34572,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35096,"parent_record_offset":35068,"ancestor_record_offset":34572,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35124,"parent_record_offset":35096,"ancestor_record_offset":34572,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35068,"parent_record_offset":35040,"ancestor_record_offset":34572,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":35040,"parent_record_offset":35012,"ancestor_record_offset":34572,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":35012,"parent_record_offset":34968,"ancestor_record_offset":34572,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":36352,"parent_record_offset":36328,"ancestor_record_offset":34572,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":35216,"parent_record_offset":35180,"ancestor_record_offset":34572,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":513,"record_offset":35592,"parent_record_offset":35548,"ancestor_record_offset":34572,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":513,"record_offset":36268,"parent_record_offset":36244,"ancestor_record_offset":34572,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":35456,"parent_record_offset":35432,"ancestor_record_offset":34572,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":36156,"parent_record_offset":36132,"ancestor_record_offset":34572,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":35432,"parent_record_offset":35180,"ancestor_record_offset":34572,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":34852,"parent_record_offset":34828,"ancestor_record_offset":34572,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":513,"record_offset":35548,"parent_record_offset":34896,"ancestor_record_offset":34572,"inlinee":24881,"source_line":229,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":35520,"parent_record_offset":34896,"ancestor_record_offset":34572,"inlinee":26796,"source_line":211,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":34968,"parent_record_offset":34896,"ancestor_record_offset":34572,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":36244,"parent_record_offset":34896,"ancestor_record_offset":34572,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":36132,"parent_record_offset":34896,"ancestor_record_offset":34572,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_camera_mode_gizmos::cursor_icon
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7303
// 首条位置: {"module":513,"inlinee":29443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":513,"record_offset":35548,"parent_record_offset":34896,"ancestor_record_offset":34896,"inlinee":24881,"source_line":229,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":35520,"parent_record_offset":34896,"ancestor_record_offset":34896,"inlinee":26796,"source_line":211,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":513,"record_offset":34968,"parent_record_offset":34896,"ancestor_record_offset":34896,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":36244,"parent_record_offset":34896,"ancestor_record_offset":34896,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":513,"record_offset":36132,"parent_record_offset":34896,"ancestor_record_offset":34896,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":35356,"parent_record_offset":35332,"ancestor_record_offset":34896,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":35380,"parent_record_offset":35356,"ancestor_record_offset":34896,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":35332,"parent_record_offset":35304,"ancestor_record_offset":34896,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35244,"parent_record_offset":35216,"ancestor_record_offset":34896,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35304,"parent_record_offset":35272,"ancestor_record_offset":34896,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":35272,"parent_record_offset":35244,"ancestor_record_offset":34896,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35180,"parent_record_offset":35152,"ancestor_record_offset":34896,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35152,"parent_record_offset":35124,"ancestor_record_offset":34896,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35096,"parent_record_offset":35068,"ancestor_record_offset":34896,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35124,"parent_record_offset":35096,"ancestor_record_offset":34896,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":35068,"parent_record_offset":35040,"ancestor_record_offset":34896,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":513,"record_offset":35040,"parent_record_offset":35012,"ancestor_record_offset":34896,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":513,"record_offset":35012,"parent_record_offset":34968,"ancestor_record_offset":34896,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":35216,"parent_record_offset":35180,"ancestor_record_offset":34896,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":513,"record_offset":35592,"parent_record_offset":35548,"ancestor_record_offset":34896,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":513,"record_offset":36268,"parent_record_offset":36244,"ancestor_record_offset":34896,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":35456,"parent_record_offset":35432,"ancestor_record_offset":34896,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":513,"record_offset":36156,"parent_record_offset":36132,"ancestor_record_offset":34896,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":35432,"parent_record_offset":35180,"ancestor_record_offset":34896,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_camera_mode_gizmos.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_camera_mode_gizmos.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_camera_mode_gizmos.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_camera_mode_gizmos::cursor_icon
// 信息 ID: b34c975c032ce894c5df59f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure$0; module=513, inline_record=34896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos
// 信息 ID: e5e41ac5d106fbe37fbbca50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2e62b8162aa6076220dbae4c: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)> > @ 0x2e67ba -> 0x9527a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)> > > @ 0x47a3dc -> 0x9527a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure$0
// 信息 ID: 0b4f9ba3fe4a1f4d0b174a0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x94f5c2 -> 0x952b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
