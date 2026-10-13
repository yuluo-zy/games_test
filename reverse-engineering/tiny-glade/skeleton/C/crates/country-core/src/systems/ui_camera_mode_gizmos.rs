// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ui_camera_mode_gizmos.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: eee3127e36d65def402801e998262c359c76af2dbfe5c6557ef19291b297553f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_camera_mode_gizmos.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 574
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":574,"record_offset":368,"rva":12040528,"va":5380749648}
// 编译实例 RVAs: 0xb7b950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5071403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e8d59db2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380750086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5071593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e8179db2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380750292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12027884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e8147848ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380750071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2624831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e8c1f2d7ff","size":5,"soft_fail":false,"target_va":5378125088,"va":5380749914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1140279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e837661100","size":5,"soft_fail":false,"target_va":5381890784,"va":5380750500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13146671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e82f9ac800","size":5,"soft_fail":false,"target_va":5393896944,"va":5380750268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36306827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e88bff2902","size":5,"soft_fail":false,"target_va":5417057056,"va":5380750224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35870423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e8d7562302","size":5,"soft_fail":false,"target_va":5416620769,"va":5380750341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4044801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e801b83d00","size":5,"soft_fail":false,"target_va":5384794864,"va":5380750058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35870619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380749648,"raw_hex":"e89b572302","size":5,"soft_fail":false,"target_va":5416620794,"va":5380750170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":574,"record_offset":816,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":574,"record_offset":900,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":574,"record_offset":760,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":1928,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":574,"record_offset":648,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":29412,"source_line":238,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::EscMenuState::is_open
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":574,"record_offset":560,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":36390,"source_line":161,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":574,"record_offset":620,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":32168,"source_line":440,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::main_camera::CameraRotationPivot::use_camera_cursor_grab_icon
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/main_camera.rs
//     evidence={"module":574,"record_offset":788,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":32277,"source_line":236,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":574,"record_offset":704,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":28124,"source_line":80,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":574,"record_offset":1124,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":574,"record_offset":1900,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":368,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":368,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":368,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":368,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":368,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":368,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":996,"parent_record_offset":964,"ancestor_record_offset":368,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2080,"parent_record_offset":2024,"ancestor_record_offset":368,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":964,"parent_record_offset":928,"ancestor_record_offset":368,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":368,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":574,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":368,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":368,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":368,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":368,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":1384,"parent_record_offset":1352,"ancestor_record_offset":368,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":368,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":368,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":1424,"parent_record_offset":1384,"ancestor_record_offset":368,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":368,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":368,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":368,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":368,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":574,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":368,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":574,"record_offset":1156,"parent_record_offset":1124,"ancestor_record_offset":368,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":574,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":368,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":368,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":574,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":368,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":574,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":368,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":574,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":368,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":574,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":368,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":574,"record_offset":1664,"parent_record_offset":1276,"ancestor_record_offset":368,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":368,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":1744,"parent_record_offset":1124,"ancestor_record_offset":368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":1804,"parent_record_offset":1768,"ancestor_record_offset":368,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":368,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":1860,"parent_record_offset":1768,"ancestor_record_offset":368,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::EscMenuState::is_closed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":574,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":368,"inlinee":31075,"source_line":165,"archive":"_pdb/C/modules/00574.jsonl.gz"}
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
// 首条位置: {"module":574,"record_offset":2500,"rva":12041424,"va":5380750544}
// 编译实例 RVAs: 0xb7bcd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5071936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e8c09bb2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380750619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5072394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e8f699b2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380751093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12028417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e8ff7548ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380750604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141628ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_camera_mode_gizmos.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11194503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e887d0aa00","size":5,"soft_fail":false,"target_va":5391945440,"va":5380750932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35869750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e836542302","size":5,"soft_fail":false,"target_va":5416620769,"va":5380751014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35869759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e83f542302","size":5,"soft_fail":false,"target_va":5416620794,"va":5380751030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1812059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380750544,"raw_hex":"e8a559e4ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5380750790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":3704,"parent_record_offset":2500,"ancestor_record_offset":2500,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":2676,"parent_record_offset":2500,"ancestor_record_offset":2500,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":3788,"parent_record_offset":2500,"ancestor_record_offset":2500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_camera_mode_gizmos::cursor_icon
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_camera_mode_gizmos.rs
//     evidence={"module":574,"record_offset":2856,"parent_record_offset":2500,"ancestor_record_offset":2500,"inlinee":36392,"source_line":89,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":2500,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":2500,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":2500,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":2500,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2500,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":2500,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":2736,"parent_record_offset":2704,"ancestor_record_offset":2500,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2500,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2500,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":574,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2500,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3256,"parent_record_offset":3228,"ancestor_record_offset":2500,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":2500,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":2500,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":2500,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2500,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3164,"parent_record_offset":3136,"ancestor_record_offset":2500,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":2500,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":2500,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":574,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":2500,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":574,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2500,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2500,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":3228,"parent_record_offset":3192,"ancestor_record_offset":2500,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":574,"record_offset":3024,"parent_record_offset":2980,"ancestor_record_offset":2500,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":574,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":2500,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":574,"record_offset":3472,"parent_record_offset":3192,"ancestor_record_offset":2500,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":2500,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":574,"record_offset":2980,"parent_record_offset":2856,"ancestor_record_offset":2500,"inlinee":26824,"source_line":225,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":574,"record_offset":2952,"parent_record_offset":2856,"ancestor_record_offset":2500,"inlinee":30509,"source_line":318,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":574,"record_offset":2924,"parent_record_offset":2856,"ancestor_record_offset":2500,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":3672,"parent_record_offset":2856,"ancestor_record_offset":2500,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":3560,"parent_record_offset":2856,"ancestor_record_offset":2500,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_camera_mode_gizmos::cursor_icon
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E28
// 首条位置: {"module":574,"inlinee":36392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::components::Ui2dImage::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":574,"record_offset":2980,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":26824,"source_line":225,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":574,"record_offset":2952,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":30509,"source_line":318,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":574,"record_offset":2924,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":3672,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":3560,"parent_record_offset":2856,"ancestor_record_offset":2856,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2856,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2856,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":2856,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":574,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":2856,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3256,"parent_record_offset":3228,"ancestor_record_offset":2856,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":2856,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":574,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":2856,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2856,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3164,"parent_record_offset":3136,"ancestor_record_offset":2856,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":2856,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":574,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":2856,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":574,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":2856,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":574,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2856,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":3228,"parent_record_offset":3192,"ancestor_record_offset":2856,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":574,"record_offset":3024,"parent_record_offset":2980,"ancestor_record_offset":2856,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":574,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":2856,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":574,"record_offset":3472,"parent_record_offset":3192,"ancestor_record_offset":2856,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":2856,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00574.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: b5fbdefd0f0535f67f68012b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure$0; module=574, inline_record=2856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos
// 信息 ID: 35c7cac61db3c5b56bad0c2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2e62b8162aa6076220dbae4c: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)> > @ 0x284e66 -> 0xb7b950 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)> @ 0x4d50c9 -> 0xb7b950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure$0
// 信息 ID: f7a43be857d47685d3e9b9e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::ui_camera_mode_gizmos::ui_camera_mode_gizmos::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xb84c40 -> 0xb7bcd0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
