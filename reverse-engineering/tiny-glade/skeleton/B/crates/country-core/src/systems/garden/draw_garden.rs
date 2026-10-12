// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/garden/draw_garden.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 8220e56ea507139ba6df31b9e60987113de9300447dc11e93a1f0fd082c198d0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_garden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 563, 569
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":110020,"rva":11421216,"va":5380130336}
// 编译实例 RVAs: 0xae4620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ab59e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-192922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e8660efdff","size":5,"soft_fail":false,"target_va":5379938784,"va":5380131701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1752563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e8f3bd1a00","size":5,"soft_fail":false,"target_va":5381883040,"va":5380130472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5775376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e8f0dfa7ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5380130939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31410306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e88248df01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380131753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_border::GladeBorder::is_shape_inside
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-73441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e81fe1feff","size":5,"soft_fail":false,"target_va":5380057456,"va":5380130892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1796066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e81e98e4ff","size":5,"soft_fail":false,"target_va":5378334544,"va":5380130605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1781002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380130336,"raw_hex":"e80a2d1b00","size":5,"soft_fail":false,"target_va":5381911872,"va":5380130865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":569,"record_offset":112076,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":569,"record_offset":110704,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":569,"record_offset":110304,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":569,"record_offset":110644,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::DrawState::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":569,"record_offset":111116,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":32092,"source_line":64,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::DragState::is_some
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":111220,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":28968,"source_line":355,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":110332,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":28952,"source_line":95,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":110416,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":26166,"source_line":254,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":569,"record_offset":110192,"parent_record_offset":110020,"ancestor_record_offset":110020,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":110888,"parent_record_offset":110864,"ancestor_record_offset":110020,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":110936,"parent_record_offset":110912,"ancestor_record_offset":110020,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":110912,"parent_record_offset":110888,"ancestor_record_offset":110020,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":110864,"parent_record_offset":110824,"ancestor_record_offset":110020,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":110824,"parent_record_offset":110788,"ancestor_record_offset":110020,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":569,"record_offset":110760,"parent_record_offset":110728,"ancestor_record_offset":110020,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":569,"record_offset":110728,"parent_record_offset":110704,"ancestor_record_offset":110020,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":569,"record_offset":110788,"parent_record_offset":110728,"ancestor_record_offset":110020,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":569,"record_offset":110272,"parent_record_offset":110192,"ancestor_record_offset":110020,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":569,"record_offset":110244,"parent_record_offset":110192,"ancestor_record_offset":110020,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":569,"record_offset":110216,"parent_record_offset":110192,"ancestor_record_offset":110020,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":110668,"parent_record_offset":110644,"ancestor_record_offset":110020,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":110976,"parent_record_offset":110824,"ancestor_record_offset":110020,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::finish
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":569,"record_offset":111188,"parent_record_offset":111116,"ancestor_record_offset":110020,"inlinee":32094,"source_line":43,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::start_or_continue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":569,"record_offset":111160,"parent_record_offset":111116,"ancestor_record_offset":110020,"inlinee":32093,"source_line":50,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":110356,"parent_record_offset":110332,"ancestor_record_offset":110020,"inlinee":28899,"source_line":86,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":110452,"parent_record_offset":110416,"ancestor_record_offset":110020,"inlinee":26167,"source_line":150,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":569,"record_offset":110496,"parent_record_offset":110452,"ancestor_record_offset":110020,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":569,"record_offset":110380,"parent_record_offset":110356,"ancestor_record_offset":110020,"inlinee":28898,"source_line":175,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E24
// 首条位置: {"module":563,"inlinee":32292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":563,"record_offset":87280,"parent_record_offset":87256,"ancestor_record_offset":87256,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":87400,"parent_record_offset":87368,"ancestor_record_offset":87256,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":87340,"parent_record_offset":87304,"ancestor_record_offset":87256,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":87304,"parent_record_offset":87280,"ancestor_record_offset":87256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":563,"record_offset":87368,"parent_record_offset":87304,"ancestor_record_offset":87256,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E25
// 首条位置: {"module":563,"inlinee":32293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":563,"record_offset":87476,"parent_record_offset":87452,"ancestor_record_offset":87452,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":87596,"parent_record_offset":87564,"ancestor_record_offset":87452,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":87536,"parent_record_offset":87500,"ancestor_record_offset":87452,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":87500,"parent_record_offset":87476,"ancestor_record_offset":87452,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":563,"record_offset":87564,"parent_record_offset":87500,"ancestor_record_offset":87452,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E23
// 首条位置: {"module":563,"inlinee":32291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":563,"record_offset":84732,"parent_record_offset":84708,"ancestor_record_offset":84708,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":84964,"parent_record_offset":84940,"ancestor_record_offset":84708,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":85012,"parent_record_offset":84988,"ancestor_record_offset":84708,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":84988,"parent_record_offset":84964,"ancestor_record_offset":84708,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":84940,"parent_record_offset":84900,"ancestor_record_offset":84708,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":84900,"parent_record_offset":84864,"ancestor_record_offset":84708,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":84784,"parent_record_offset":84756,"ancestor_record_offset":84708,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":563,"record_offset":84756,"parent_record_offset":84732,"ancestor_record_offset":84708,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":563,"record_offset":84840,"parent_record_offset":84816,"ancestor_record_offset":84708,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":563,"record_offset":84816,"parent_record_offset":84732,"ancestor_record_offset":84708,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":563,"record_offset":84864,"parent_record_offset":84840,"ancestor_record_offset":84708,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":563,"record_offset":85052,"parent_record_offset":84900,"ancestor_record_offset":84708,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00563.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_garden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_garden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_garden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden
// 信息 ID: 90a324d52e84170c6cb0a29c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5610e0d4cae9f755d15b6588: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)> > > @ 0x14df01c -> 0xae4620 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)> > @ 0x17681e5 -> 0xae4620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$0
// 信息 ID: 2aaba095ef8cc0bdafd608de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$0 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; module=563, inline_record=87256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$1
// 信息 ID: f4913d38e379e7acc8193c78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$1 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; module=563, inline_record=87452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$2
// 信息 ID: fe07ac745adf1ae8fec0fba3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$2 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>; module=563, inline_record=84708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
