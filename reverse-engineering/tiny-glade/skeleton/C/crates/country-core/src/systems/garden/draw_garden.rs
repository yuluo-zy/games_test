// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/garden/draw_garden.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 93bd8625ac462b14bf278e798915b4ec2829c4696e0905f4cf300d417f43a9c7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_garden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 604, 1671
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
// 首条位置: {"module":1671,"record_offset":110484,"rva":25031840,"va":5393740960}
// 编译实例 RVAs: 0x17df4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5dec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12065516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e814e547ff","size":5,"soft_fail":false,"target_va":5381676736,"va":5393742247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10467364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e8dc4760ff","size":5,"soft_fail":false,"target_va":5383273968,"va":5393741327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_garden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22609507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e863fe5801","size":5,"soft_fail":false,"target_va":5416351504,"va":5393741992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22879904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e8a01e5d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5393742299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::DrawState::update_bounded_by_glade
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12330672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e850d943ff","size":5,"soft_fail":false,"target_va":5381411104,"va":5393741771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15619747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e85da911ff","size":5,"soft_fail":false,"target_va":5378121712,"va":5393741454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10435791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393740960,"raw_hex":"e831c360ff","size":5,"soft_fail":false,"target_va":5383305920,"va":5393741706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1671,"record_offset":112060,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1671,"record_offset":111640,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1671,"record_offset":110972,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1671,"record_offset":110796,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":1671,"record_offset":111380,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::DrawState::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":1671,"record_offset":111172,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":29012,"source_line":64,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::temp_eraser::TempEraser::brush_size_override
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":1671,"record_offset":110748,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":28976,"source_line":48,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::temp_eraser::TempEraser::effective_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":1671,"record_offset":110660,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":29007,"source_line":38,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1671,"record_offset":111000,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":28328,"source_line":110,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1671,"record_offset":111084,"parent_record_offset":110484,"ancestor_record_offset":110484,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1671,"record_offset":111868,"parent_record_offset":111836,"ancestor_record_offset":110484,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1671,"record_offset":111916,"parent_record_offset":111892,"ancestor_record_offset":110484,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1671,"record_offset":111892,"parent_record_offset":111868,"ancestor_record_offset":110484,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1671,"record_offset":111836,"parent_record_offset":111796,"ancestor_record_offset":110484,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1671,"record_offset":111764,"parent_record_offset":111728,"ancestor_record_offset":110484,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1671,"record_offset":111796,"parent_record_offset":111764,"ancestor_record_offset":110484,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1671,"record_offset":111700,"parent_record_offset":111664,"ancestor_record_offset":110484,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1671,"record_offset":111664,"parent_record_offset":111640,"ancestor_record_offset":110484,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1671,"record_offset":111728,"parent_record_offset":111664,"ancestor_record_offset":110484,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110932,"parent_record_offset":110852,"ancestor_record_offset":110484,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110904,"parent_record_offset":110852,"ancestor_record_offset":110484,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110876,"parent_record_offset":110852,"ancestor_record_offset":110484,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1671,"record_offset":111404,"parent_record_offset":111380,"ancestor_record_offset":110484,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1671,"record_offset":111956,"parent_record_offset":111796,"ancestor_record_offset":110484,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::garden::draw_garden::draw_garden::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/garden/draw_garden.rs
//     evidence={"module":1671,"record_offset":110824,"parent_record_offset":110796,"ancestor_record_offset":110484,"inlinee":82561,"source_line":64,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::finish
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":1671,"record_offset":111236,"parent_record_offset":111172,"ancestor_record_offset":110484,"inlinee":29016,"source_line":43,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::start_or_continue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":1671,"record_offset":111208,"parent_record_offset":111172,"ancestor_record_offset":110484,"inlinee":29013,"source_line":50,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::temp_eraser::TempEraser::eraser_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":1671,"record_offset":110704,"parent_record_offset":110660,"ancestor_record_offset":110484,"inlinee":29008,"source_line":25,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1671,"record_offset":111024,"parent_record_offset":111000,"ancestor_record_offset":110484,"inlinee":28117,"source_line":101,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1671,"record_offset":111108,"parent_record_offset":111084,"ancestor_record_offset":110484,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1671,"record_offset":111136,"parent_record_offset":111108,"ancestor_record_offset":110484,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":1671,"record_offset":111048,"parent_record_offset":111024,"ancestor_record_offset":110484,"inlinee":28119,"source_line":183,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1671,"record_offset":110852,"parent_record_offset":110824,"ancestor_record_offset":110484,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14281
// 首条位置: {"module":1671,"inlinee":82561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1671,"record_offset":110852,"parent_record_offset":110824,"ancestor_record_offset":110824,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110932,"parent_record_offset":110852,"ancestor_record_offset":110824,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110904,"parent_record_offset":110852,"ancestor_record_offset":110824,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1671,"record_offset":110876,"parent_record_offset":110852,"ancestor_record_offset":110824,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9625
// 首条位置: {"module":604,"inlinee":38437}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":604,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":18628,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":18772,"parent_record_offset":18740,"ancestor_record_offset":18628,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":18804,"parent_record_offset":18772,"ancestor_record_offset":18628,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":18712,"parent_record_offset":18676,"ancestor_record_offset":18628,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":18676,"parent_record_offset":18652,"ancestor_record_offset":18628,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":604,"record_offset":18740,"parent_record_offset":18676,"ancestor_record_offset":18628,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9626
// 首条位置: {"module":604,"inlinee":38438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":604,"record_offset":18884,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":19004,"parent_record_offset":18972,"ancestor_record_offset":18860,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":19036,"parent_record_offset":19004,"ancestor_record_offset":18860,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":18944,"parent_record_offset":18908,"ancestor_record_offset":18860,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":18908,"parent_record_offset":18884,"ancestor_record_offset":18860,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":604,"record_offset":18972,"parent_record_offset":18908,"ancestor_record_offset":18860,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_garden::draw_garden::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9624
// 首条位置: {"module":604,"inlinee":38436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":604,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":16636,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":604,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":16636,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":604,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":16636,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":604,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":16636,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":16900,"parent_record_offset":16860,"ancestor_record_offset":16636,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":16828,"parent_record_offset":16792,"ancestor_record_offset":16636,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":604,"record_offset":16860,"parent_record_offset":16828,"ancestor_record_offset":16636,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":16712,"parent_record_offset":16684,"ancestor_record_offset":16636,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":604,"record_offset":16684,"parent_record_offset":16660,"ancestor_record_offset":16636,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":604,"record_offset":16768,"parent_record_offset":16744,"ancestor_record_offset":16636,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":604,"record_offset":16744,"parent_record_offset":16660,"ancestor_record_offset":16636,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":604,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":16636,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00604.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":604,"record_offset":17012,"parent_record_offset":16860,"ancestor_record_offset":16636,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00604.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::ui_edit_terrain::TerrainStrokeParamsCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CursorCaptureCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_edit_terrain::TerrainSteepnessEditAmount>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden
// 信息 ID: d9db79274fd139a6c31273ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9f3334b004b008908b67acbb: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)> > @ 0x1930830 -> 0x17df4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >)> @ 0x1ac317b -> 0x17df4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$0
// 信息 ID: c9b8d07318fc53cd631891fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::draw_garden::draw_garden; module=1671, inline_record=110824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$1
// 信息 ID: b03e7139ce7a54c40f570b34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$1 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; module=604, inline_record=18628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$2
// 信息 ID: 6ab25dd5a8a6c8a19ad3af04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$2 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; module=604, inline_record=18860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_garden::draw_garden::closure$3
// 信息 ID: 3a79cd95bd0043b868172867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::garden::draw_garden::draw_garden", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::garden::draw_garden::draw_garden::closure_env$3 <- country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>; module=604, inline_record=16636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
