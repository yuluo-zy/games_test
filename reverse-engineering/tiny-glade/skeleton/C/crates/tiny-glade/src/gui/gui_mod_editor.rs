// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/gui_mod_editor.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6356eb5921e06b0fda5aa9c98ec04bbd02d41f3212245a6e77771b395b5ef216
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_mod_editor.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 28, 128, 149, 192, 208, 228, 252, 253
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.11"}]
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use ecolor; // namespace candidate
//   package identities: [{"name":"ecolor","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use emath; // namespace candidate
//   package identities: [{"name":"emath","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use epaint; // namespace candidate
//   package identities: [{"name":"epaint","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editing_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":2232,"rva":4204528,"va":5372913648}
// 编译实例 RVAs: 0x4027f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t9898356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e874099700","size":5,"soft_fail":false,"target_va":5382812128,"va":5372913767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2764289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e8012e2a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5372914394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2764112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e8502d2a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372914587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4192192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e84008c0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372914379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054ca00
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1352125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e8bda11400","size":5,"soft_fail":false,"target_va":5374265856,"va":5372913726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054cb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1351859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e8b3a01400","size":5,"soft_fail":false,"target_va":5374266160,"va":5372914296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607c50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2118453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e835532000","size":5,"soft_fail":false,"target_va":5375032400,"va":5372913942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069b010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2720956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e8bc842900","size":5,"soft_fail":false,"target_va":5375635472,"va":5372914511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43705627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e81be59a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5372915137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t9858107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e83b6c9600","size":5,"soft_fail":false,"target_va":5382771904,"va":5372913792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::ask_to_delete
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6925844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e814ae6900","size":5,"soft_fail":false,"target_va":5379840496,"va":5372914647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::open_sound_board
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6931495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e827c46900","size":5,"soft_fail":false,"target_va":5379846208,"va":5372914708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::set_category
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6925900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372913648,"raw_hex":"e84cae6900","size":5,"soft_fail":false,"target_va":5379840464,"va":5372914559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":128,"record_offset":3456,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":2512,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::auto_shrink
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":128,"record_offset":3580,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":13246,"source_line":385,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::id_source
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":128,"record_offset":3484,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":7366,"source_line":286,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":128,"record_offset":3608,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":7368,"source_line":665,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::colored_label
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":128,"record_offset":3064,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":7357,"source_line":1305,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":128,"record_offset":2452,"parent_record_offset":2232,"ancestor_record_offset":2232,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":128,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2232,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":2232,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2232,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":2232,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":2232,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":2232,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":2232,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":3708,"parent_record_offset":3676,"ancestor_record_offset":2232,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":3984,"parent_record_offset":3928,"ancestor_record_offset":2232,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":128,"record_offset":3676,"parent_record_offset":3640,"ancestor_record_offset":2232,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":128,"record_offset":3640,"parent_record_offset":3608,"ancestor_record_offset":2232,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":2232,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":2232,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":2232,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":2616,"parent_record_offset":2580,"ancestor_record_offset":2232,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2232,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":128,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2232,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":2532,"parent_record_offset":2512,"ancestor_record_offset":2232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":128,"record_offset":2648,"parent_record_offset":2616,"ancestor_record_offset":2232,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":2676,"parent_record_offset":2616,"ancestor_record_offset":2232,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":2708,"parent_record_offset":2580,"ancestor_record_offset":2232,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":128,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2232,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":128,"record_offset":3144,"parent_record_offset":3064,"ancestor_record_offset":2232,"inlinee":7358,"source_line":254,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$0::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":128,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2232,"inlinee":7354,"source_line":44,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":128,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2232,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::register_fonts
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C20
// 首条位置: {"module":28,"inlinee":7200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":4248,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":3476,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":2880,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":7203,"source_line":718,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":1176,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4332,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":28,"record_offset":3216,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":5228,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3036,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":6651,"source_line":2145,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3120,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":1688,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] epaint::text::fonts::FontData::from_static
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":1660,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":7202,"source_line":123,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1012,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":1012,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1012,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1012,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":1012,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1012,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":1012,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1888,"parent_record_offset":1832,"ancestor_record_offset":1012,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":1012,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":1012,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":2948,"parent_record_offset":2880,"ancestor_record_offset":1012,"inlinee":7204,"source_line":1121,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":1012,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":1012,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":1012,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1012,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":1012,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":1012,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":1012,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":1012,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":1012,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":3932,"parent_record_offset":3896,"ancestor_record_offset":1012,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":1012,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1248,"parent_record_offset":1204,"ancestor_record_offset":1012,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":1012,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":1012,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":1012,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5404,"parent_record_offset":5376,"ancestor_record_offset":1012,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5376,"parent_record_offset":5340,"ancestor_record_offset":1012,"inlinee":7216,"source_line":2245,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":1012,"inlinee":7215,"source_line":2260,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5652,"parent_record_offset":5404,"ancestor_record_offset":1012,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":1012,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5440,"parent_record_offset":5404,"ancestor_record_offset":1012,"inlinee":7219,"source_line":2249,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$31::from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":1012,"inlinee":7214,"source_line":2352,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$70::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":1012,"inlinee":7212,"source_line":3903,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$72::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":1012,"inlinee":7210,"source_line":3941,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4404,"parent_record_offset":4368,"ancestor_record_offset":1012,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":1012,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":1012,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4452,"parent_record_offset":4404,"ancestor_record_offset":1012,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":1340,"parent_record_offset":1304,"ancestor_record_offset":1012,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":1012,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":1012,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":3156,"parent_record_offset":3120,"ancestor_record_offset":1012,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":28,"record_offset":5188,"parent_record_offset":4404,"ancestor_record_offset":1012,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":28,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":1012,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":28,"record_offset":4368,"parent_record_offset":4332,"ancestor_record_offset":1012,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":28,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":1012,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":28,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":1012,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":28,"record_offset":3316,"parent_record_offset":3272,"ancestor_record_offset":1012,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":1012,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cloned.rs
//     evidence={"module":28,"record_offset":4616,"parent_record_offset":4580,"ancestor_record_offset":1012,"inlinee":6736,"source_line":57,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":28,"record_offset":4652,"parent_record_offset":4616,"ancestor_record_offset":1012,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":28,"record_offset":4892,"parent_record_offset":4688,"ancestor_record_offset":1012,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":4580,"parent_record_offset":4404,"ancestor_record_offset":1012,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":4944,"parent_record_offset":4892,"ancestor_record_offset":1012,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":1012,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":1012,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":28,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":1012,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":28,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1012,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":28,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":1012,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":2912,"parent_record_offset":2880,"ancestor_record_offset":1012,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1012,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":1584,"parent_record_offset":1304,"ancestor_record_offset":1012,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":3400,"parent_record_offset":3340,"ancestor_record_offset":1012,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":1012,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":1012,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":1012,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":1012,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":28,"record_offset":5588,"parent_record_offset":5404,"ancestor_record_offset":1012,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":1012,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":28,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":1012,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":28,"record_offset":4688,"parent_record_offset":4652,"ancestor_record_offset":1012,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editor_style
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C53
// 首条位置: {"module":28,"inlinee":7251}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17824,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":6168,"source_line":2729,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::map::impl$29::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17852,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":6172,"source_line":1811,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":19028,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":19656,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::style::impl$40::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":16180,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":7253,"source_line":135,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] emath::vec2::impl$20::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/vec2.rs
//     evidence={"module":28,"record_offset":19876,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":7286,"source_line":417,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":15996,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":15996,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":19344,"parent_record_offset":19316,"ancestor_record_offset":15996,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":15996,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17056,"parent_record_offset":16968,"ancestor_record_offset":15996,"inlinee":6680,"source_line":696,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":16968,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":6679,"source_line":227,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":28,"record_offset":18120,"parent_record_offset":18052,"ancestor_record_offset":15996,"inlinee":4906,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":28,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":15996,"inlinee":4926,"source_line":21,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::init_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17944,"parent_record_offset":17900,"ancestor_record_offset":15996,"inlinee":4912,"source_line":221,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17900,"parent_record_offset":17852,"ancestor_record_offset":15996,"inlinee":6173,"source_line":172,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::impl$23::next_unchecked::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18168,"parent_record_offset":18120,"ancestor_record_offset":15996,"inlinee":6175,"source_line":559,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18540,"parent_record_offset":18444,"ancestor_record_offset":15996,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18484,"parent_record_offset":18444,"ancestor_record_offset":15996,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv_valmut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18844,"parent_record_offset":18052,"ancestor_record_offset":15996,"inlinee":6177,"source_line":1151,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18216,"parent_record_offset":18168,"ancestor_record_offset":15996,"inlinee":6176,"source_line":380,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18444,"parent_record_offset":18168,"ancestor_record_offset":15996,"inlinee":4910,"source_line":715,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18052,"parent_record_offset":17900,"ancestor_record_offset":15996,"inlinee":7282,"source_line":558,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18720,"parent_record_offset":18444,"ancestor_record_offset":15996,"inlinee":4911,"source_line":811,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18252,"parent_record_offset":18216,"ancestor_record_offset":15996,"inlinee":4913,"source_line":893,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18320,"parent_record_offset":18216,"ancestor_record_offset":15996,"inlinee":4917,"source_line":328,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::first_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17976,"parent_record_offset":17944,"ancestor_record_offset":15996,"inlinee":4915,"source_line":630,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18008,"parent_record_offset":17976,"ancestor_record_offset":15996,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::into_key_val_mut_at
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":15996,"inlinee":6178,"source_line":531,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18284,"parent_record_offset":18252,"ancestor_record_offset":15996,"inlinee":4914,"source_line":287,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$16::ascend::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":15996,"inlinee":4919,"source_line":340,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19204,"parent_record_offset":19176,"ancestor_record_offset":15996,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19176,"parent_record_offset":19140,"ancestor_record_offset":15996,"inlinee":7216,"source_line":2245,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19140,"parent_record_offset":19112,"ancestor_record_offset":15996,"inlinee":7215,"source_line":2260,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19456,"parent_record_offset":19204,"ancestor_record_offset":15996,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":16392,"parent_record_offset":16364,"ancestor_record_offset":15996,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19424,"parent_record_offset":19392,"ancestor_record_offset":15996,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19244,"parent_record_offset":19204,"ancestor_record_offset":15996,"inlinee":7219,"source_line":2249,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$31::from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19112,"parent_record_offset":19084,"ancestor_record_offset":15996,"inlinee":7214,"source_line":2352,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":16364,"parent_record_offset":16340,"ancestor_record_offset":15996,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":16828,"parent_record_offset":16800,"ancestor_record_offset":15996,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$70::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19084,"parent_record_offset":19056,"ancestor_record_offset":15996,"inlinee":7212,"source_line":3903,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$72::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19056,"parent_record_offset":19028,"ancestor_record_offset":15996,"inlinee":7210,"source_line":3941,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":17008,"parent_record_offset":16968,"ancestor_record_offset":15996,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":18376,"parent_record_offset":18320,"ancestor_record_offset":15996,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":18752,"parent_record_offset":18168,"ancestor_record_offset":15996,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":16300,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$3::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":15996,"inlinee":7284,"source_line":1537,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":19608,"parent_record_offset":19140,"ancestor_record_offset":15996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":16716,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":19484,"parent_record_offset":19456,"ancestor_record_offset":15996,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$3::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":18952,"parent_record_offset":18868,"ancestor_record_offset":15996,"inlinee":6180,"source_line":1913,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":16424,"parent_record_offset":16392,"ancestor_record_offset":15996,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":15996,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":28,"record_offset":19392,"parent_record_offset":19204,"ancestor_record_offset":15996,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":18596,"parent_record_offset":18540,"ancestor_record_offset":15996,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":28,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":15996,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":28,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":15996,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":16864,"parent_record_offset":16828,"ancestor_record_offset":15996,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":16452,"parent_record_offset":16392,"ancestor_record_offset":15996,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":16888,"parent_record_offset":16864,"ancestor_record_offset":15996,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":16920,"parent_record_offset":16828,"ancestor_record_offset":15996,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::impl$4::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ecolor","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":28,"record_offset":17480,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7271,"source_line":13,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$32::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":16340,"parent_record_offset":16300,"ancestor_record_offset":15996,"inlinee":7255,"source_line":20,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$44::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17240,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":7261,"source_line":256,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$54::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17272,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":7263,"source_line":618,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$58::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17300,"parent_record_offset":16180,"ancestor_record_offset":15996,"inlinee":7265,"source_line":656,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$63::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17452,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7269,"source_line":818,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$69::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17760,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7279,"source_line":827,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$74::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17420,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7267,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$86::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17788,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7281,"source_line":1946,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::shadow::impl$2::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/shadow.rs
//     evidence={"module":28,"record_offset":17564,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7275,"source_line":10,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::shape::impl$60::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/shape.rs
//     evidence={"module":28,"record_offset":17536,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7273,"source_line":749,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::stroke::impl$4::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/stroke.rs
//     evidence={"module":28,"record_offset":17620,"parent_record_offset":17300,"ancestor_record_offset":15996,"inlinee":7277,"source_line":8,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::text::fonts::impl$15::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":16532,"parent_record_offset":16496,"ancestor_record_offset":15996,"inlinee":7257,"source_line":16,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::text::fonts::impl$19::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":16556,"parent_record_offset":16532,"ancestor_record_offset":15996,"inlinee":7259,"source_line":70,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor
// 记录形式: compiled_procedure; instances=1
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":28,"record_offset":536,"rva":1375504,"va":5370084624}
// 编译实例 RVAs: 0x14fd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t14709894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e88674e000","size":5,"soft_fail":false,"target_va":5384795984,"va":5370086085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..btree..map..BTreeMap$LT$K$C$V$C$A$GT$$u20$as$u20$core..clone..Clone$GT$::clone::clone_subtree::h466a050992cfbe19 (.llvm.14268497953171331515)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1966131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e833001e00","size":5,"soft_fail":false,"target_va":5372055440,"va":5370089304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$epaint..text..fonts..FontDefinitions$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12765148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8dcc7c200","size":5,"soft_fail":false,"target_va":5382850000,"va":5370084847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t5593816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8d85a5500","size":5,"soft_fail":false,"target_va":5375678688,"va":5370084867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t5593640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8285a5500","size":5,"soft_fail":false,"target_va":5375678704,"va":5370085059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1362665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e81735ebff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370084852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008a9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-816190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8c28bf3ff","size":5,"soft_fail":false,"target_va":5369276848,"va":5370093033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008e5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-800222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e822caf3ff","size":5,"soft_fail":false,"target_va":5369292192,"va":5370092409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-789850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8a6f2f3ff","size":5,"soft_fail":false,"target_va":5369302464,"va":5370092309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400992b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-755690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e81678f4ff","size":5,"soft_fail":false,"target_va":5369336496,"va":5370092181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e910
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-734012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8c4ccf4ff","size":5,"soft_fail":false,"target_va":5369358608,"va":5370092615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a67f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-702317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e89348f5ff","size":5,"soft_fail":false,"target_va":5369391088,"va":5370093400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-661143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e869e9f5ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5370092498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6360
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-636605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e84349f6ff","size":5,"soft_fail":false,"target_va":5369455456,"va":5370092056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401550d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e885370000","size":5,"soft_fail":false,"target_va":5370106064,"va":5370091846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fb580
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1750436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8a4b51a00","size":5,"soft_fail":false,"target_va":5371835776,"va":5370085335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1768376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8b8fb1a00","size":5,"soft_fail":false,"target_va":5371853200,"va":5370084819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140336ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1993490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8126b1e00","size":5,"soft_fail":false,"target_va":5372078784,"va":5370085289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2612896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8a0de2700","size":5,"soft_fail":false,"target_va":5372699568,"va":5370086667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140402450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2824029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e85d172b00","size":5,"soft_fail":false,"target_va":5372912720,"va":5370088686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040daf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2872146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e852d32b00","size":5,"soft_fail":false,"target_va":5372959472,"va":5370087321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3395387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e83bcf3300","size":5,"soft_fail":false,"target_va":5373482032,"va":5370086640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404916b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3411259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e83b0d3400","size":5,"soft_fail":false,"target_va":5373499056,"va":5370087792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054cb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4177965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e82dc03f00","size":5,"soft_fail":false,"target_va":5374266160,"va":5370088190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4930283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8eb3a4b00","size":5,"soft_fail":false,"target_va":5375018384,"va":5370088096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4931221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8953e4b00","size":5,"soft_fail":false,"target_va":5375019776,"va":5370088550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=7
//     target_aliases=77 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5744219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e85ba65700","size":5,"soft_fail":false,"target_va":5375829056,"va":5370084832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb490
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8042290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e832b77a00","size":5,"soft_fail":false,"target_va":5378126992,"va":5370084697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa7450
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9795277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8cd769500","size":5,"soft_fail":false,"target_va":5379880016,"va":5370084734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46264344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e818f0c102","size":5,"soft_fail":false,"target_va":5416351392,"va":5370087043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46267009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e881fac102","size":5,"soft_fail":false,"target_va":5416353040,"va":5370086026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46970126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e80eb5cc02","size":5,"soft_fail":false,"target_va":5417057040,"va":5370086909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46971497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e869bacc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370085554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t46529517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8edfbc502","size":5,"soft_fail":false,"target_va":5416620769,"va":5370091247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1958155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e80be11d00","size":5,"soft_fail":false,"target_va":5372043168,"va":5370085008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14707454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8fe6ae000","size":5,"soft_fail":false,"target_va":5384794864,"va":5370087405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t46534616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8d80fc602","size":5,"soft_fail":false,"target_va":5416620794,"va":5370086173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t12683307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e82b88c100","size":5,"soft_fail":false,"target_va":5382771904,"va":5370088592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14709066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e84a71e000","size":5,"soft_fail":false,"target_va":5384794736,"va":5370085665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t46530247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8c7fec502","size":5,"soft_fail":false,"target_va":5416621600,"va":5370091348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterId::mod_item
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9328350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8de568e00","size":5,"soft_fail":false,"target_va":5379414928,"va":5370086573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::log
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9759722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8eaeb9400","size":5,"soft_fail":false,"target_va":5379847168,"va":5370087441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::ModEditor::close
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9791268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e824679500","size":5,"soft_fail":false,"target_va":5379880032,"va":5370088759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::set_fonts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12660652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8ac2fc100","size":5,"soft_fail":false,"target_va":5382749632,"va":5370088975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::style
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12597975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370084624,"raw_hex":"e8d73ac000","size":5,"soft_fail":false,"target_va":5382685712,"va":5370087732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":14368,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":13392,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":19936,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":13308,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":7356,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":14452,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":736,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":15220,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":14852,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":28,"record_offset":14548,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7248,"source_line":70,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::resizable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":28,"record_offset":14580,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7249,"source_line":299,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":28,"record_offset":14608,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7250,"source_line":365,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":28,"record_offset":14480,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::editor_style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":15996,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7251,"source_line":90,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::item_statuses
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":764,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7197,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::log_new_failures
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":7412,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7225,"source_line":703,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::register_fonts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":1012,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7200,"source_line":57,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":28,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":536,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":1888,"parent_record_offset":1832,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":4248,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":14644,"parent_record_offset":14608,"ancestor_record_offset":536,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":3476,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":536,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":2880,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":7203,"source_line":718,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17824,"parent_record_offset":15996,"ancestor_record_offset":536,"inlinee":6168,"source_line":2729,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17056,"parent_record_offset":16968,"ancestor_record_offset":536,"inlinee":6680,"source_line":696,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$29::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":17852,"parent_record_offset":15996,"ancestor_record_offset":536,"inlinee":6172,"source_line":1811,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":28,"record_offset":16968,"parent_record_offset":16180,"ancestor_record_offset":536,"inlinee":6679,"source_line":227,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":28,"record_offset":18120,"parent_record_offset":18052,"ancestor_record_offset":536,"inlinee":4906,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":28,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":536,"inlinee":4926,"source_line":21,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::init_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17944,"parent_record_offset":17900,"ancestor_record_offset":536,"inlinee":4912,"source_line":221,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17900,"parent_record_offset":17852,"ancestor_record_offset":536,"inlinee":6173,"source_line":172,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::impl$23::next_unchecked::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18168,"parent_record_offset":18120,"ancestor_record_offset":536,"inlinee":6175,"source_line":559,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18540,"parent_record_offset":18444,"ancestor_record_offset":536,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18484,"parent_record_offset":18444,"ancestor_record_offset":536,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":2948,"parent_record_offset":2880,"ancestor_record_offset":536,"inlinee":7204,"source_line":1121,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv_valmut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18844,"parent_record_offset":18052,"ancestor_record_offset":536,"inlinee":6177,"source_line":1151,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18216,"parent_record_offset":18168,"ancestor_record_offset":536,"inlinee":6176,"source_line":380,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18444,"parent_record_offset":18168,"ancestor_record_offset":536,"inlinee":4910,"source_line":715,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":18052,"parent_record_offset":17900,"ancestor_record_offset":536,"inlinee":7282,"source_line":558,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18720,"parent_record_offset":18444,"ancestor_record_offset":536,"inlinee":4911,"source_line":811,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18252,"parent_record_offset":18216,"ancestor_record_offset":536,"inlinee":4913,"source_line":893,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18320,"parent_record_offset":18216,"ancestor_record_offset":536,"inlinee":4917,"source_line":328,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::first_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":28,"record_offset":17976,"parent_record_offset":17944,"ancestor_record_offset":536,"inlinee":4915,"source_line":630,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18008,"parent_record_offset":17976,"ancestor_record_offset":536,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::into_key_val_mut_at
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":536,"inlinee":6178,"source_line":531,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18284,"parent_record_offset":18252,"ancestor_record_offset":536,"inlinee":4914,"source_line":287,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$16::ascend::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":28,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":536,"inlinee":4919,"source_line":340,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":10540,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":536,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7796,"parent_record_offset":7764,"ancestor_record_offset":536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":536,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":536,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":536,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":536,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":536,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":536,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":3932,"parent_record_offset":3896,"ancestor_record_offset":536,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":536,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1248,"parent_record_offset":1204,"ancestor_record_offset":536,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":536,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":536,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":536,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":1176,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":28,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":536,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5404,"parent_record_offset":5376,"ancestor_record_offset":536,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5376,"parent_record_offset":5340,"ancestor_record_offset":536,"inlinee":7216,"source_line":2245,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":536,"inlinee":7215,"source_line":2260,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5652,"parent_record_offset":5404,"ancestor_record_offset":536,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":15016,"parent_record_offset":14980,"ancestor_record_offset":536,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":536,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5440,"parent_record_offset":5404,"ancestor_record_offset":536,"inlinee":7219,"source_line":2249,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$31::from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":536,"inlinee":7214,"source_line":2352,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":16364,"parent_record_offset":16340,"ancestor_record_offset":536,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":14980,"parent_record_offset":14952,"ancestor_record_offset":536,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$70::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":536,"inlinee":7212,"source_line":3903,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$72::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":536,"inlinee":7210,"source_line":3941,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7764,"parent_record_offset":7716,"ancestor_record_offset":536,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4404,"parent_record_offset":4368,"ancestor_record_offset":536,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":536,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":980,"parent_record_offset":764,"ancestor_record_offset":536,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7680,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7716,"parent_record_offset":7680,"ancestor_record_offset":536,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4452,"parent_record_offset":4404,"ancestor_record_offset":536,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":1340,"parent_record_offset":1304,"ancestor_record_offset":536,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":536,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":10020,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4332,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":536,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":536,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":3156,"parent_record_offset":3120,"ancestor_record_offset":536,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":11932,"parent_record_offset":11244,"ancestor_record_offset":536,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":536,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":536,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":10076,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":536,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":28,"record_offset":9912,"parent_record_offset":9884,"ancestor_record_offset":536,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":28,"record_offset":5188,"parent_record_offset":4404,"ancestor_record_offset":536,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":28,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":536,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":28,"record_offset":4368,"parent_record_offset":4332,"ancestor_record_offset":536,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":536,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":28,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":536,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":28,"record_offset":3216,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":28,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":536,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":28,"record_offset":3316,"parent_record_offset":3272,"ancestor_record_offset":536,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":536,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":9812,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":536,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":536,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":5228,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":28,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":536,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":28,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":536,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":10624,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":9640,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":536,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cloned.rs
//     evidence={"module":28,"record_offset":4616,"parent_record_offset":4580,"ancestor_record_offset":536,"inlinee":6736,"source_line":57,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":28,"record_offset":4652,"parent_record_offset":4616,"ancestor_record_offset":536,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":28,"record_offset":4892,"parent_record_offset":4688,"ancestor_record_offset":536,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":536,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7920,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":536,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":536,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":28,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":536,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":28,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":536,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":28,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":536,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":896,"parent_record_offset":764,"ancestor_record_offset":536,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":4580,"parent_record_offset":4404,"ancestor_record_offset":536,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":4944,"parent_record_offset":4892,"ancestor_record_offset":536,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":7540,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":536,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":536,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":28,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":536,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":28,"record_offset":9288,"parent_record_offset":9260,"ancestor_record_offset":536,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":28,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":536,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":28,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":28,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":536,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":2912,"parent_record_offset":2880,"ancestor_record_offset":536,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3036,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":6651,"source_line":2145,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":536,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":536,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":18752,"parent_record_offset":18168,"ancestor_record_offset":536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":8332,"parent_record_offset":8048,"ancestor_record_offset":536,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":3120,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":536,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":16300,"parent_record_offset":16180,"ancestor_record_offset":536,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":1584,"parent_record_offset":1304,"ancestor_record_offset":536,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":536,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":3400,"parent_record_offset":3340,"ancestor_record_offset":536,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$3::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":536,"inlinee":7284,"source_line":1537,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":536,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=86
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":1688,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":536,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":536,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":536,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":536,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$3::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":18952,"parent_record_offset":18868,"ancestor_record_offset":536,"inlinee":6180,"source_line":1913,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":15048,"parent_record_offset":15016,"ancestor_record_offset":536,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":536,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":10140,"parent_record_offset":10076,"ancestor_record_offset":536,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":536,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":536,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":536,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":28,"record_offset":5588,"parent_record_offset":5404,"ancestor_record_offset":536,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":536,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":536,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":536,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":536,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":812,"parent_record_offset":764,"ancestor_record_offset":536,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":28,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":536,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":28,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":536,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":28,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":536,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":28,"record_offset":7984,"parent_record_offset":7948,"ancestor_record_offset":536,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":28,"record_offset":4688,"parent_record_offset":4652,"ancestor_record_offset":536,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":28,"record_offset":8088,"parent_record_offset":8048,"ancestor_record_offset":536,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8144,"parent_record_offset":8116,"ancestor_record_offset":536,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":536,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":536,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":16864,"parent_record_offset":16828,"ancestor_record_offset":536,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":13356,"parent_record_offset":13332,"ancestor_record_offset":536,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":15076,"parent_record_offset":15016,"ancestor_record_offset":536,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":15108,"parent_record_offset":14980,"ancestor_record_offset":536,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":28,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":536,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::impl$4::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ecolor","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":28,"record_offset":17480,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7271,"source_line":13,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$32::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":16340,"parent_record_offset":16300,"ancestor_record_offset":536,"inlinee":7255,"source_line":20,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$40::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":16180,"parent_record_offset":15996,"ancestor_record_offset":536,"inlinee":7253,"source_line":135,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$44::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17240,"parent_record_offset":16180,"ancestor_record_offset":536,"inlinee":7261,"source_line":256,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$54::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17272,"parent_record_offset":16180,"ancestor_record_offset":536,"inlinee":7263,"source_line":618,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$58::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17300,"parent_record_offset":16180,"ancestor_record_offset":536,"inlinee":7265,"source_line":656,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$63::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17452,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7269,"source_line":818,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$69::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17760,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7279,"source_line":827,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$74::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17420,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7267,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::style::impl$86::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/style.rs
//     evidence={"module":28,"record_offset":17788,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7281,"source_line":1946,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] emath::vec2::impl$20::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/vec2.rs
//     evidence={"module":28,"record_offset":19876,"parent_record_offset":15996,"ancestor_record_offset":536,"inlinee":7286,"source_line":417,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::shadow::impl$2::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/shadow.rs
//     evidence={"module":28,"record_offset":17564,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7275,"source_line":10,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::shape::impl$60::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/shape.rs
//     evidence={"module":28,"record_offset":17536,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7273,"source_line":749,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::stroke::impl$4::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/stroke.rs
//     evidence={"module":28,"record_offset":17620,"parent_record_offset":17300,"ancestor_record_offset":536,"inlinee":7277,"source_line":8,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::text::fonts::FontData::from_static
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":1660,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":7202,"source_line":123,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::text::fonts::impl$15::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":16532,"parent_record_offset":16496,"ancestor_record_offset":536,"inlinee":7257,"source_line":16,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] epaint::text::fonts::impl$19::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/text/fonts.rs
//     evidence={"module":28,"record_offset":16556,"parent_record_offset":16532,"ancestor_record_offset":536,"inlinee":7259,"source_line":70,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":536,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9260,"parent_record_offset":9228,"ancestor_record_offset":536,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9200,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9060,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9668,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9144,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":28,"record_offset":9004,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":28,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":536,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":536,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":536,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":536,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":9468,"parent_record_offset":9360,"ancestor_record_offset":536,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":536,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":536,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9760,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":536,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":536,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8900,"parent_record_offset":8840,"ancestor_record_offset":536,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":536,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9032,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9360,"parent_record_offset":8948,"ancestor_record_offset":536,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":8816,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":10200,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":8364,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::item_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":8048,"parent_record_offset":7412,"ancestor_record_offset":536,"inlinee":7232,"source_line":696,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":28,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":536,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D99
// 首条位置: {"module":192,"inlinee":15769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":192,"record_offset":7316,"parent_record_offset":6836,"ancestor_record_offset":6836,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::preview_position
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":6836,"inlinee":7375,"source_line":727,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7544,"parent_record_offset":7512,"ancestor_record_offset":6836,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":6836,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":6836,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7512,"parent_record_offset":7472,"ancestor_record_offset":6836,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7440,"parent_record_offset":7404,"ancestor_record_offset":6836,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7472,"parent_record_offset":7440,"ancestor_record_offset":6836,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":192,"record_offset":7376,"parent_record_offset":7340,"ancestor_record_offset":6836,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":192,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":6836,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":192,"record_offset":7404,"parent_record_offset":7340,"ancestor_record_offset":6836,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7100,"parent_record_offset":7044,"ancestor_record_offset":6836,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6836,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":6836,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":7284,"parent_record_offset":6864,"ancestor_record_offset":6836,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":7632,"parent_record_offset":7472,"ancestor_record_offset":6836,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":192,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":6836,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":6988,"parent_record_offset":6952,"ancestor_record_offset":6836,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":7132,"parent_record_offset":6952,"ancestor_record_offset":6836,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":192,"record_offset":7244,"parent_record_offset":6952,"ancestor_record_offset":6836,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6924,"parent_record_offset":6892,"ancestor_record_offset":6836,"inlinee":7184,"source_line":729,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CA9
// 首条位置: {"module":28,"inlinee":7337}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":30972,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":33772,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7350,"source_line":1414,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29500,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":30384,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":29884,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::separator
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":30664,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7345,"source_line":1559,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::set_enabled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":29608,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7341,"source_line":265,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::log_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":44864,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7364,"source_line":503,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::new_mod_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":31032,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7348,"source_line":258,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::publish_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":34224,"parent_record_offset":29388,"ancestor_record_offset":29388,"inlinee":7351,"source_line":392,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":28,"record_offset":45268,"parent_record_offset":45244,"ancestor_record_offset":29388,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":29992,"parent_record_offset":29968,"ancestor_record_offset":29388,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":30016,"parent_record_offset":29992,"ancestor_record_offset":29388,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41800,"parent_record_offset":41776,"ancestor_record_offset":29388,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41824,"parent_record_offset":41800,"ancestor_record_offset":29388,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":29388,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41848,"parent_record_offset":41824,"ancestor_record_offset":29388,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":29968,"parent_record_offset":29944,"ancestor_record_offset":29388,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41776,"parent_record_offset":41720,"ancestor_record_offset":29388,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":29944,"parent_record_offset":29916,"ancestor_record_offset":29388,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":29916,"parent_record_offset":29884,"ancestor_record_offset":29388,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":37216,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":29388,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41748,"parent_record_offset":41720,"ancestor_record_offset":29388,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41720,"parent_record_offset":41696,"ancestor_record_offset":29388,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41696,"parent_record_offset":41672,"ancestor_record_offset":29388,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$5::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":37188,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7362,"source_line":529,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":28,"record_offset":37752,"parent_record_offset":37724,"ancestor_record_offset":29388,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":29816,"parent_record_offset":29792,"ancestor_record_offset":29388,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":29792,"parent_record_offset":29752,"ancestor_record_offset":29388,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":30488,"parent_record_offset":30460,"ancestor_record_offset":29388,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":35168,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":28,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":29388,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":31000,"parent_record_offset":30972,"ancestor_record_offset":29388,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":33848,"parent_record_offset":33824,"ancestor_record_offset":29388,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":29688,"parent_record_offset":29664,"ancestor_record_offset":29388,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":32260,"parent_record_offset":32236,"ancestor_record_offset":29388,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":37300,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":28,"record_offset":33824,"parent_record_offset":33796,"ancestor_record_offset":29388,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":29388,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29544,"parent_record_offset":29524,"ancestor_record_offset":29388,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29568,"parent_record_offset":29544,"ancestor_record_offset":29388,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":29388,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29712,"parent_record_offset":29688,"ancestor_record_offset":29388,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=111
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":30408,"parent_record_offset":30384,"ancestor_record_offset":29388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":29840,"parent_record_offset":29816,"ancestor_record_offset":29388,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":37808,"parent_record_offset":37780,"ancestor_record_offset":29388,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":37836,"parent_record_offset":37808,"ancestor_record_offset":29388,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":30584,"parent_record_offset":30524,"ancestor_record_offset":29388,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":30616,"parent_record_offset":30488,"ancestor_record_offset":29388,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":28,"record_offset":37160,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::scroll_area::ScrollArea::id_source
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45220,"parent_record_offset":44864,"ancestor_record_offset":29388,"inlinee":7366,"source_line":286,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::scroll_area::ScrollArea::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45344,"parent_record_offset":44864,"ancestor_record_offset":29388,"inlinee":7368,"source_line":665,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::scroll_area::ScrollArea::stick_to_bottom
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45316,"parent_record_offset":44864,"ancestor_record_offset":29388,"inlinee":7367,"source_line":423,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":31696,"parent_record_offset":31672,"ancestor_record_offset":29388,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":28,"record_offset":45244,"parent_record_offset":45220,"ancestor_record_offset":29388,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::Layout::prefer_right_to_left
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":30280,"parent_record_offset":30252,"ancestor_record_offset":29388,"inlinee":7296,"source_line":333,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::impl$16::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":30312,"parent_record_offset":30280,"ancestor_record_offset":29388,"inlinee":7298,"source_line":78,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::painter::Painter::ctx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/painter.rs
//     evidence={"module":28,"record_offset":31508,"parent_record_offset":31476,"ancestor_record_offset":29388,"inlinee":6082,"source_line":106,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::painter::Painter::is_visible
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/painter.rs
//     evidence={"module":28,"record_offset":29664,"parent_record_offset":29640,"ancestor_record_offset":29388,"inlinee":7343,"source_line":92,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::placer::Placer::prefer_right_to_left
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/placer.rs
//     evidence={"module":28,"record_offset":30252,"parent_record_offset":30092,"ancestor_record_offset":29388,"inlinee":7294,"source_line":47,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":31644,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":31672,"parent_record_offset":31644,"ancestor_record_offset":29388,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31544,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::colored_label
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":32236,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":7357,"source_line":1305,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::ctx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31476,"parent_record_offset":31452,"ancestor_record_offset":29388,"inlinee":6080,"source_line":228,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":34468,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7352,"source_line":1316,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":35252,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::horizontal_with_main_wrap_dyn
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":30092,"parent_record_offset":29884,"ancestor_record_offset":29388,"inlinee":7291,"source_line":1964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31452,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":6084,"source_line":398,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::is_visible
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":29640,"parent_record_offset":29608,"ancestor_record_offset":29388,"inlinee":7342,"source_line":275,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31112,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::spacing
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":30140,"parent_record_offset":30092,"ancestor_record_offset":29388,"inlinee":7292,"source_line":187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::text_edit_singleline
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31424,"parent_record_offset":31032,"ancestor_record_offset":29388,"inlinee":7349,"source_line":1404,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::visuals
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":29752,"parent_record_offset":29608,"ancestor_record_offset":29388,"inlinee":7344,"source_line":207,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":32316,"parent_record_offset":32236,"ancestor_record_offset":29388,"inlinee":7358,"source_line":254,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":34556,"parent_record_offset":34468,"ancestor_record_offset":29388,"inlinee":7355,"source_line":167,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":37428,"parent_record_offset":37404,"ancestor_record_offset":29388,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text_style
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":34588,"parent_record_offset":34556,"ancestor_record_offset":29388,"inlinee":7356,"source_line":153,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$0::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":29388,"inlinee":7354,"source_line":44,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":29388,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":31576,"parent_record_offset":31544,"ancestor_record_offset":29388,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":31136,"parent_record_offset":31112,"ancestor_record_offset":29388,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::separator::impl$0::default
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/separator.rs
//     evidence={"module":28,"record_offset":30688,"parent_record_offset":30664,"ancestor_record_offset":29388,"inlinee":7347,"source_line":22,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] emath::align::impl$11::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/align.rs
//     evidence={"module":28,"record_offset":30340,"parent_record_offset":30280,"ancestor_record_offset":29388,"inlinee":7300,"source_line":6,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":29524,"parent_record_offset":29500,"ancestor_record_offset":29388,"inlinee":7340,"source_line":207,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::format_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":39380,"parent_record_offset":34224,"ancestor_record_offset":29388,"inlinee":7363,"source_line":522,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CAC
// 首条位置: {"module":28,"inlinee":7340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29544,"parent_record_offset":29524,"ancestor_record_offset":29524,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":29568,"parent_record_offset":29544,"ancestor_record_offset":29524,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCE
// 首条位置: {"module":28,"inlinee":7374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":51612,"parent_record_offset":50848,"ancestor_record_offset":50848,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":51108,"parent_record_offset":50848,"ancestor_record_offset":50848,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":51020,"parent_record_offset":50848,"ancestor_record_offset":50848,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":50924,"parent_record_offset":50848,"ancestor_record_offset":50848,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":51224,"parent_record_offset":51188,"ancestor_record_offset":50848,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":51188,"parent_record_offset":51160,"ancestor_record_offset":50848,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":28,"record_offset":51640,"parent_record_offset":51612,"ancestor_record_offset":50848,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":50984,"parent_record_offset":50952,"ancestor_record_offset":50848,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":51256,"parent_record_offset":51224,"ancestor_record_offset":50848,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":51284,"parent_record_offset":51224,"ancestor_record_offset":50848,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":51316,"parent_record_offset":51188,"ancestor_record_offset":50848,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":51072,"parent_record_offset":51048,"ancestor_record_offset":50848,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":51048,"parent_record_offset":51020,"ancestor_record_offset":50848,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":50952,"parent_record_offset":50924,"ancestor_record_offset":50848,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::new_mod_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CB4
// 首条位置: {"module":28,"inlinee":7348}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":31172,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":31644,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31544,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::colored_label
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":32236,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7357,"source_line":1305,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31452,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":6084,"source_line":398,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31112,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::text_edit_singleline
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31424,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7349,"source_line":1404,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":31288,"parent_record_offset":31252,"ancestor_record_offset":31032,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":31252,"parent_record_offset":31224,"ancestor_record_offset":31032,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":32260,"parent_record_offset":32236,"ancestor_record_offset":31032,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":31032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":31320,"parent_record_offset":31288,"ancestor_record_offset":31032,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":31348,"parent_record_offset":31288,"ancestor_record_offset":31032,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":31380,"parent_record_offset":31252,"ancestor_record_offset":31032,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":31696,"parent_record_offset":31672,"ancestor_record_offset":31032,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::painter::Painter::ctx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/painter.rs
//     evidence={"module":28,"record_offset":31508,"parent_record_offset":31476,"ancestor_record_offset":31032,"inlinee":6082,"source_line":106,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":31672,"parent_record_offset":31644,"ancestor_record_offset":31032,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::ctx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":31476,"parent_record_offset":31452,"ancestor_record_offset":31032,"inlinee":6080,"source_line":228,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":32316,"parent_record_offset":32236,"ancestor_record_offset":31032,"inlinee":7358,"source_line":254,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$0::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":31032,"inlinee":7354,"source_line":44,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":31576,"parent_record_offset":31544,"ancestor_record_offset":31032,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":31136,"parent_record_offset":31112,"ancestor_record_offset":31032,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::new_mod_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x388E
// 首条位置: {"module":149,"inlinee":14478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editing_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4684
// 首条位置: {"module":253,"inlinee":18052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":6332,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] egui::grid::Grid::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/grid.rs
//     evidence={"module":253,"record_offset":5944,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":8082,"source_line":308,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] egui::grid::Grid::num_columns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/grid.rs
//     evidence={"module":253,"record_offset":6040,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":14605,"source_line":333,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] egui::grid::Grid::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/grid.rs
//     evidence={"module":253,"record_offset":6096,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":8083,"source_line":394,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] egui::grid::Grid::spacing
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/grid.rs
//     evidence={"module":253,"record_offset":6068,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":14606,"source_line":379,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":253,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5896,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":5896,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5896,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5896,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":5896,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":253,"record_offset":6168,"parent_record_offset":6132,"ancestor_record_offset":5896,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":253,"record_offset":6132,"parent_record_offset":6096,"ancestor_record_offset":5896,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":253,"record_offset":6472,"parent_record_offset":6436,"ancestor_record_offset":5896,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":253,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":5896,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":5896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":253,"record_offset":6504,"parent_record_offset":6472,"ancestor_record_offset":5896,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":6532,"parent_record_offset":6472,"ancestor_record_offset":5896,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":6564,"parent_record_offset":6436,"ancestor_record_offset":5896,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":253,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":5896,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D9B
// 首条位置: {"module":192,"inlinee":15771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":192,"record_offset":16596,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":192,"record_offset":13972,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":192,"record_offset":10876,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":16124,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":14368,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":9052,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":192,"record_offset":16068,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":192,"record_offset":10792,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::combo_box::ComboBox::selected_text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/combo_box.rs
//     evidence={"module":192,"record_offset":11276,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":15775,"source_line":107,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::combo_box::ComboBox::show_ui
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/combo_box.rs
//     evidence={"module":192,"record_offset":13456,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":15776,"source_line":161,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":192,"record_offset":15180,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":15076,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::colored_label
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":16212,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":7357,"source_line":1305,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":14396,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":10088,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::strong
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":8912,"parent_record_offset":8728,"ancestor_record_offset":8728,"inlinee":15772,"source_line":1344,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":11916,"parent_record_offset":11892,"ancestor_record_offset":8728,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":8728,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":8728,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":8728,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":8728,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":8728,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":11892,"parent_record_offset":11744,"ancestor_record_offset":8728,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":12320,"parent_record_offset":12264,"ancestor_record_offset":8728,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":192,"record_offset":13536,"parent_record_offset":13496,"ancestor_record_offset":8728,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":192,"record_offset":13496,"parent_record_offset":13456,"ancestor_record_offset":8728,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":192,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":8728,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":8728,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":12292,"parent_record_offset":12264,"ancestor_record_offset":8728,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":8728,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":11744,"parent_record_offset":11712,"ancestor_record_offset":8728,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":11712,"parent_record_offset":11684,"ancestor_record_offset":8728,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":8728,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":11780,"parent_record_offset":11744,"ancestor_record_offset":8728,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":192,"record_offset":11556,"parent_record_offset":11524,"ancestor_record_offset":8728,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":192,"record_offset":11588,"parent_record_offset":11556,"ancestor_record_offset":8728,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":192,"record_offset":11620,"parent_record_offset":11588,"ancestor_record_offset":8728,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":192,"record_offset":11524,"parent_record_offset":11492,"ancestor_record_offset":8728,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":192,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":8728,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":192,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":8728,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":192,"record_offset":9168,"parent_record_offset":9132,"ancestor_record_offset":8728,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":192,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":8728,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":11656,"parent_record_offset":11620,"ancestor_record_offset":8728,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":192,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":8728,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":192,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":8728,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":192,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":8728,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":192,"record_offset":11828,"parent_record_offset":11804,"ancestor_record_offset":8728,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":192,"record_offset":16152,"parent_record_offset":16124,"ancestor_record_offset":8728,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":16176,"parent_record_offset":16152,"ancestor_record_offset":8728,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":16620,"parent_record_offset":16596,"ancestor_record_offset":8728,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":192,"record_offset":12020,"parent_record_offset":11620,"ancestor_record_offset":8728,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":8728,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":192,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":8728,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":192,"record_offset":9200,"parent_record_offset":9168,"ancestor_record_offset":8728,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":192,"record_offset":10932,"parent_record_offset":10904,"ancestor_record_offset":8728,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":192,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":8728,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":192,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":8728,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":192,"record_offset":10904,"parent_record_offset":10876,"ancestor_record_offset":8728,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":192,"record_offset":9228,"parent_record_offset":9168,"ancestor_record_offset":8728,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":192,"record_offset":9260,"parent_record_offset":9132,"ancestor_record_offset":8728,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":192,"record_offset":13996,"parent_record_offset":13972,"ancestor_record_offset":8728,"inlinee":15778,"source_line":630,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":192,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":8728,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::Layout::prefer_right_to_left
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":192,"record_offset":14700,"parent_record_offset":14672,"ancestor_record_offset":8728,"inlinee":7296,"source_line":333,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::impl$16::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":192,"record_offset":14732,"parent_record_offset":14700,"ancestor_record_offset":8728,"inlinee":7298,"source_line":78,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::placer::Placer::prefer_right_to_left
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/placer.rs
//     evidence={"module":192,"record_offset":14672,"parent_record_offset":14624,"ancestor_record_offset":8728,"inlinee":7294,"source_line":47,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":192,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":8728,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::horizontal_with_main_wrap_dyn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":14624,"parent_record_offset":14396,"ancestor_record_offset":8728,"inlinee":7291,"source_line":1964,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":16292,"parent_record_offset":16212,"ancestor_record_offset":8728,"inlinee":7358,"source_line":254,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":11392,"parent_record_offset":11360,"ancestor_record_offset":8728,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::strong
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":8992,"parent_record_offset":8912,"ancestor_record_offset":8728,"inlinee":15773,"source_line":186,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$0::from
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":8728,"inlinee":7354,"source_line":44,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":8728,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$8::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":11360,"parent_record_offset":11328,"ancestor_record_offset":8728,"inlinee":8130,"source_line":693,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":192,"record_offset":15108,"parent_record_offset":15076,"ancestor_record_offset":8728,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":192,"record_offset":9020,"parent_record_offset":8912,"ancestor_record_offset":8728,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] emath::align::impl$11::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/align.rs
//     evidence={"module":192,"record_offset":14760,"parent_record_offset":14700,"ancestor_record_offset":8728,"inlinee":7300,"source_line":6,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x404A
// 首条位置: {"module":208,"inlinee":16458}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":208,"record_offset":25120,"parent_record_offset":24588,"ancestor_record_offset":24588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::selectable_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":208,"record_offset":24616,"parent_record_offset":24588,"ancestor_record_offset":24588,"inlinee":16427,"source_line":1541,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":208,"record_offset":24980,"parent_record_offset":24944,"ancestor_record_offset":24588,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":208,"record_offset":24944,"parent_record_offset":24920,"ancestor_record_offset":24588,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":208,"record_offset":24872,"parent_record_offset":24616,"ancestor_record_offset":24588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":208,"record_offset":25012,"parent_record_offset":24980,"ancestor_record_offset":24588,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":208,"record_offset":25040,"parent_record_offset":24980,"ancestor_record_offset":24588,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":208,"record_offset":25072,"parent_record_offset":24944,"ancestor_record_offset":24588,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$23::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":208,"record_offset":24728,"parent_record_offset":24616,"ancestor_record_offset":24588,"inlinee":15778,"source_line":630,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":208,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":24588,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":208,"record_offset":24756,"parent_record_offset":24616,"ancestor_record_offset":24588,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":208,"record_offset":24784,"parent_record_offset":24756,"ancestor_record_offset":24588,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::mark_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":208,"record_offset":24844,"parent_record_offset":24616,"ancestor_record_offset":24588,"inlinee":16431,"source_line":512,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::selectable_label
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":208,"record_offset":24664,"parent_record_offset":24616,"ancestor_record_offset":24588,"inlinee":16428,"source_line":1531,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::selected_label::SelectableLabel::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/selected_label.rs
//     evidence={"module":208,"record_offset":24692,"parent_record_offset":24664,"ancestor_record_offset":24588,"inlinee":15865,"source_line":31,"archive":"_pdb/C/modules/00208.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D98
// 首条位置: {"module":192,"inlinee":15768}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":192,"record_offset":5352,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":192,"record_offset":6336,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":5520,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":192,"record_offset":5432,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":192,"record_offset":5224,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6836,"parent_record_offset":5164,"ancestor_record_offset":5164,"inlinee":15769,"source_line":177,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":5164,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5164,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5164,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":192,"record_offset":5948,"parent_record_offset":5892,"ancestor_record_offset":5164,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":192,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":5164,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7544,"parent_record_offset":7512,"ancestor_record_offset":5164,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":5920,"parent_record_offset":5892,"ancestor_record_offset":5164,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5164,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":5164,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":5164,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":192,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":5164,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":192,"record_offset":5636,"parent_record_offset":5600,"ancestor_record_offset":5164,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":192,"record_offset":5600,"parent_record_offset":5572,"ancestor_record_offset":5164,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7512,"parent_record_offset":7472,"ancestor_record_offset":5164,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7440,"parent_record_offset":7404,"ancestor_record_offset":5164,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":192,"record_offset":7472,"parent_record_offset":7440,"ancestor_record_offset":5164,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":192,"record_offset":7376,"parent_record_offset":7340,"ancestor_record_offset":5164,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":192,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":5164,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":192,"record_offset":7316,"parent_record_offset":6836,"ancestor_record_offset":5164,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":192,"record_offset":7404,"parent_record_offset":7340,"ancestor_record_offset":5164,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":192,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":5164,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7100,"parent_record_offset":7044,"ancestor_record_offset":5164,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":5164,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":5164,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":5164,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":7284,"parent_record_offset":6864,"ancestor_record_offset":5164,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":5164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":192,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":5164,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":192,"record_offset":7632,"parent_record_offset":7472,"ancestor_record_offset":5164,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":192,"record_offset":5696,"parent_record_offset":5636,"ancestor_record_offset":5164,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":192,"record_offset":5728,"parent_record_offset":5600,"ancestor_record_offset":5164,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::audio::clutter_audio::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/clutter_audio.rs
//     evidence={"module":192,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5164,"inlinee":13265,"source_line":11,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":192,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":5164,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":192,"record_offset":5460,"parent_record_offset":5432,"ancestor_record_offset":5164,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":192,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":5164,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":192,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":5164,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":192,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":5164,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":6988,"parent_record_offset":6952,"ancestor_record_offset":5164,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":7132,"parent_record_offset":6952,"ancestor_record_offset":5164,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":192,"record_offset":7244,"parent_record_offset":6952,"ancestor_record_offset":5164,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":5164,"inlinee":7375,"source_line":727,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6924,"parent_record_offset":6892,"ancestor_record_offset":5164,"inlinee":7184,"source_line":729,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::publish_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CB7
// 首条位置: {"module":28,"inlinee":7351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":37216,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":37188,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7362,"source_line":529,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":35168,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":37724,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":37300,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":40104,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":34916,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":28,"record_offset":37160,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":38532,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":38504,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":34468,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7352,"source_line":1316,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":35252,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":35196,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::format_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":39380,"parent_record_offset":34224,"ancestor_record_offset":34224,"inlinee":7363,"source_line":522,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":35360,"parent_record_offset":35336,"ancestor_record_offset":34224,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":35384,"parent_record_offset":35360,"ancestor_record_offset":34224,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41800,"parent_record_offset":41776,"ancestor_record_offset":34224,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41824,"parent_record_offset":41800,"ancestor_record_offset":34224,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":35408,"parent_record_offset":35384,"ancestor_record_offset":34224,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41848,"parent_record_offset":41824,"ancestor_record_offset":34224,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":35336,"parent_record_offset":35312,"ancestor_record_offset":34224,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":41776,"parent_record_offset":41720,"ancestor_record_offset":34224,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":35312,"parent_record_offset":35284,"ancestor_record_offset":34224,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":35284,"parent_record_offset":35252,"ancestor_record_offset":34224,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":39412,"parent_record_offset":39380,"ancestor_record_offset":34224,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":34224,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41748,"parent_record_offset":41720,"ancestor_record_offset":34224,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41720,"parent_record_offset":41696,"ancestor_record_offset":34224,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":41696,"parent_record_offset":41672,"ancestor_record_offset":34224,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":28,"record_offset":37752,"parent_record_offset":37724,"ancestor_record_offset":34224,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":34740,"parent_record_offset":34704,"ancestor_record_offset":34224,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":35532,"parent_record_offset":35508,"ancestor_record_offset":34224,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":34704,"parent_record_offset":34676,"ancestor_record_offset":34224,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":28,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":34224,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":34500,"parent_record_offset":34468,"ancestor_record_offset":34224,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":34224,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":34628,"parent_record_offset":34588,"ancestor_record_offset":34224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":34772,"parent_record_offset":34740,"ancestor_record_offset":34224,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":37808,"parent_record_offset":37780,"ancestor_record_offset":34224,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":37836,"parent_record_offset":37808,"ancestor_record_offset":34224,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":34800,"parent_record_offset":34740,"ancestor_record_offset":34224,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":34832,"parent_record_offset":34704,"ancestor_record_offset":34224,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":38584,"parent_record_offset":38560,"ancestor_record_offset":34224,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::Layout::prefer_right_to_left
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":35648,"parent_record_offset":35620,"ancestor_record_offset":34224,"inlinee":7296,"source_line":333,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::impl$16::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":35680,"parent_record_offset":35648,"ancestor_record_offset":34224,"inlinee":7298,"source_line":78,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::placer::Placer::prefer_right_to_left
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/placer.rs
//     evidence={"module":28,"record_offset":35620,"parent_record_offset":35460,"ancestor_record_offset":34224,"inlinee":7294,"source_line":47,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":38560,"parent_record_offset":38532,"ancestor_record_offset":34224,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::horizontal_with_main_wrap_dyn
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":35460,"parent_record_offset":35252,"ancestor_record_offset":34224,"inlinee":7291,"source_line":1964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::spacing
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":35508,"parent_record_offset":35460,"ancestor_record_offset":34224,"inlinee":7292,"source_line":187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":34556,"parent_record_offset":34468,"ancestor_record_offset":34224,"inlinee":7355,"source_line":167,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":37428,"parent_record_offset":37404,"ancestor_record_offset":34224,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text_style
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":34588,"parent_record_offset":34556,"ancestor_record_offset":34224,"inlinee":7356,"source_line":153,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$0::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":34524,"parent_record_offset":34500,"ancestor_record_offset":34224,"inlinee":7354,"source_line":44,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":34224,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":40168,"parent_record_offset":40136,"ancestor_record_offset":34224,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":34884,"parent_record_offset":34468,"ancestor_record_offset":34224,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] emath::align::impl$11::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/align.rs
//     evidence={"module":28,"record_offset":35708,"parent_record_offset":35648,"ancestor_record_offset":34224,"inlinee":7300,"source_line":6,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::publish_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCB
// 首条位置: {"module":28,"inlinee":7371}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":48796,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":49164,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":48096,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":48880,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":49692,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":49588,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":49192,"parent_record_offset":48044,"ancestor_record_offset":48044,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48604,"parent_record_offset":48580,"ancestor_record_offset":48044,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":48044,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":49056,"parent_record_offset":49032,"ancestor_record_offset":48044,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":49080,"parent_record_offset":49056,"ancestor_record_offset":48044,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48652,"parent_record_offset":48628,"ancestor_record_offset":48044,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":49104,"parent_record_offset":49080,"ancestor_record_offset":48044,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48580,"parent_record_offset":48432,"ancestor_record_offset":48044,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":49032,"parent_record_offset":48976,"ancestor_record_offset":48044,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":28,"record_offset":48200,"parent_record_offset":48124,"ancestor_record_offset":48044,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":48844,"parent_record_offset":48820,"ancestor_record_offset":48044,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48376,"parent_record_offset":48352,"ancestor_record_offset":48044,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":49004,"parent_record_offset":48976,"ancestor_record_offset":48044,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48976,"parent_record_offset":48952,"ancestor_record_offset":48044,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48432,"parent_record_offset":48400,"ancestor_record_offset":48044,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48400,"parent_record_offset":48376,"ancestor_record_offset":48044,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":48044,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48468,"parent_record_offset":48432,"ancestor_record_offset":48044,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48276,"parent_record_offset":48252,"ancestor_record_offset":48044,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48300,"parent_record_offset":48276,"ancestor_record_offset":48044,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48324,"parent_record_offset":48300,"ancestor_record_offset":48044,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48252,"parent_record_offset":48228,"ancestor_record_offset":48044,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":48228,"parent_record_offset":48200,"ancestor_record_offset":48044,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":49452,"parent_record_offset":49416,"ancestor_record_offset":48044,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":49416,"parent_record_offset":49388,"ancestor_record_offset":48044,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":48352,"parent_record_offset":48324,"ancestor_record_offset":48044,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":28,"record_offset":48492,"parent_record_offset":48468,"ancestor_record_offset":48044,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":28,"record_offset":49244,"parent_record_offset":49216,"ancestor_record_offset":48044,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":48540,"parent_record_offset":48516,"ancestor_record_offset":48044,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":48516,"parent_record_offset":48492,"ancestor_record_offset":48044,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":48820,"parent_record_offset":48796,"ancestor_record_offset":48044,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":48708,"parent_record_offset":48324,"ancestor_record_offset":48044,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":48044,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":48904,"parent_record_offset":48880,"ancestor_record_offset":48044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":49484,"parent_record_offset":49452,"ancestor_record_offset":48044,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":49512,"parent_record_offset":49452,"ancestor_record_offset":48044,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":49544,"parent_record_offset":49416,"ancestor_record_offset":48044,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":49740,"parent_record_offset":49716,"ancestor_record_offset":48044,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":49716,"parent_record_offset":49692,"ancestor_record_offset":48044,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":49292,"parent_record_offset":49268,"ancestor_record_offset":48044,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":28,"record_offset":49268,"parent_record_offset":49244,"ancestor_record_offset":48044,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":49620,"parent_record_offset":49588,"ancestor_record_offset":48044,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":49216,"parent_record_offset":49192,"ancestor_record_offset":48044,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":28,"record_offset":48144,"parent_record_offset":48124,"ancestor_record_offset":48044,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":28,"record_offset":48168,"parent_record_offset":48144,"ancestor_record_offset":48044,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::publish_view::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":48124,"parent_record_offset":48096,"ancestor_record_offset":48044,"inlinee":7373,"source_line":407,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::publish_view::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 407
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCD
// 首条位置: {"module":28,"inlinee":7373}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":28,"record_offset":48200,"parent_record_offset":48124,"ancestor_record_offset":48124,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":28,"record_offset":48144,"parent_record_offset":48124,"ancestor_record_offset":48124,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48604,"parent_record_offset":48580,"ancestor_record_offset":48124,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":48124,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48652,"parent_record_offset":48628,"ancestor_record_offset":48124,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":48580,"parent_record_offset":48432,"ancestor_record_offset":48124,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48376,"parent_record_offset":48352,"ancestor_record_offset":48124,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48432,"parent_record_offset":48400,"ancestor_record_offset":48124,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48400,"parent_record_offset":48376,"ancestor_record_offset":48124,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":48468,"parent_record_offset":48432,"ancestor_record_offset":48124,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48276,"parent_record_offset":48252,"ancestor_record_offset":48124,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48300,"parent_record_offset":48276,"ancestor_record_offset":48124,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48324,"parent_record_offset":48300,"ancestor_record_offset":48124,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":28,"record_offset":48252,"parent_record_offset":48228,"ancestor_record_offset":48124,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":28,"record_offset":48228,"parent_record_offset":48200,"ancestor_record_offset":48124,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":48352,"parent_record_offset":48324,"ancestor_record_offset":48124,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":28,"record_offset":48492,"parent_record_offset":48468,"ancestor_record_offset":48124,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":48540,"parent_record_offset":48516,"ancestor_record_offset":48124,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":48516,"parent_record_offset":48492,"ancestor_record_offset":48124,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":48708,"parent_record_offset":48324,"ancestor_record_offset":48124,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":48124,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":28,"record_offset":48168,"parent_record_offset":48144,"ancestor_record_offset":48124,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::publish_view::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CA4
// 首条位置: {"module":28,"inlinee":7332}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":28576,"parent_record_offset":27796,"ancestor_record_offset":27796,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":27900,"parent_record_offset":27796,"ancestor_record_offset":27796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":28236,"parent_record_offset":27796,"ancestor_record_offset":27796,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":28140,"parent_record_offset":27796,"ancestor_record_offset":27796,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":27840,"parent_record_offset":27796,"ancestor_record_offset":27796,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":28624,"parent_record_offset":28600,"ancestor_record_offset":27796,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":28004,"parent_record_offset":27968,"ancestor_record_offset":27796,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":27968,"parent_record_offset":27944,"ancestor_record_offset":27796,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":28600,"parent_record_offset":28576,"ancestor_record_offset":27796,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":27920,"parent_record_offset":27900,"ancestor_record_offset":27796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":28036,"parent_record_offset":28004,"ancestor_record_offset":27796,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":28064,"parent_record_offset":28004,"ancestor_record_offset":27796,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":28096,"parent_record_offset":27968,"ancestor_record_offset":27796,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":28288,"parent_record_offset":28264,"ancestor_record_offset":27796,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":28264,"parent_record_offset":28236,"ancestor_record_offset":27796,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":28168,"parent_record_offset":28140,"ancestor_record_offset":27796,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":27796,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::log_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 503
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC4
// 首条位置: {"module":28,"inlinee":7364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":44968,"parent_record_offset":44864,"ancestor_record_offset":44864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::id_source
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45220,"parent_record_offset":44864,"ancestor_record_offset":44864,"inlinee":7366,"source_line":286,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45344,"parent_record_offset":44864,"ancestor_record_offset":44864,"inlinee":7368,"source_line":665,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::scroll_area::ScrollArea::stick_to_bottom
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/scroll_area.rs
//     evidence={"module":28,"record_offset":45316,"parent_record_offset":44864,"ancestor_record_offset":44864,"inlinee":7367,"source_line":423,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":44908,"parent_record_offset":44864,"ancestor_record_offset":44864,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":28,"record_offset":45268,"parent_record_offset":45244,"ancestor_record_offset":44864,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":45464,"parent_record_offset":45440,"ancestor_record_offset":44864,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":45488,"parent_record_offset":45464,"ancestor_record_offset":44864,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":45512,"parent_record_offset":45488,"ancestor_record_offset":44864,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":45440,"parent_record_offset":45408,"ancestor_record_offset":44864,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":45408,"parent_record_offset":45372,"ancestor_record_offset":44864,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":45372,"parent_record_offset":45344,"ancestor_record_offset":44864,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":45084,"parent_record_offset":45048,"ancestor_record_offset":44864,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":45048,"parent_record_offset":45020,"ancestor_record_offset":44864,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":44992,"parent_record_offset":44968,"ancestor_record_offset":44864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":45116,"parent_record_offset":45084,"ancestor_record_offset":44864,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":45144,"parent_record_offset":45084,"ancestor_record_offset":44864,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":45176,"parent_record_offset":45048,"ancestor_record_offset":44864,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":28,"record_offset":45244,"parent_record_offset":45220,"ancestor_record_offset":44864,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":44932,"parent_record_offset":44908,"ancestor_record_offset":44864,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::log_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 515
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4680
// 首条位置: {"module":253,"inlinee":18048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":3388,"parent_record_offset":3244,"ancestor_record_offset":3244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":253,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":3244,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":253,"record_offset":3332,"parent_record_offset":3244,"ancestor_record_offset":3244,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":253,"record_offset":3496,"parent_record_offset":3460,"ancestor_record_offset":3244,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":253,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":3244,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":3244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":253,"record_offset":3528,"parent_record_offset":3496,"ancestor_record_offset":3244,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":253,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3244,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":3556,"parent_record_offset":3496,"ancestor_record_offset":3244,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":3588,"parent_record_offset":3460,"ancestor_record_offset":3244,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":253,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":3244,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::format_size
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 522
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC3
// 首条位置: {"module":28,"inlinee":7363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":39412,"parent_record_offset":39380,"ancestor_record_offset":39380,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":39460,"parent_record_offset":39436,"ancestor_record_offset":39380,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":39436,"parent_record_offset":39412,"ancestor_record_offset":39380,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::modal_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 531
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":536,"rva":4203600,"va":5372912720}
// 编译实例 RVAs: 0x402450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11883197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8bd52b500","size":5,"soft_fail":false,"target_va":5384795984,"va":5372912782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2765555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8f3322a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5372913128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2765292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8ec312a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372913407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4190926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8320dc0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372913113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e910
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3554956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e874c1c9ff","size":5,"soft_fail":false,"target_va":5369358608,"va":5372913559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2106230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e876232000","size":5,"soft_fail":false,"target_va":5375019040,"va":5372912805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2106564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8c4242000","size":5,"soft_fail":false,"target_va":5375019776,"va":5372913207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44144216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e85896a102","size":5,"soft_fail":false,"target_va":5417057056,"va":5372912835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43707327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8bfeb9a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5372913437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9858559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8ff6d9600","size":5,"soft_fail":false,"target_va":5382771904,"va":5372913340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::confirm_modal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6927398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e826b46900","size":5,"soft_fail":false,"target_va":5379840768,"va":5372913365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::EditingMod::dismiss_modal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6930043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e87bbe6900","size":5,"soft_fail":false,"target_va":5379843424,"va":5372913376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::move_to_top
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9745593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372912720,"raw_hex":"e8b9b49400","size":5,"soft_fail":false,"target_va":5382658912,"va":5372913314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":1100,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":128,"record_offset":776,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":13243,"source_line":239,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":128,"record_offset":860,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7250,"source_line":365,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::title_bar
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":128,"record_offset":744,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":13242,"source_line":315,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":972,"parent_record_offset":936,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":1532,"parent_record_offset":1476,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":128,"record_offset":936,"parent_record_offset":896,"ancestor_record_offset":536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":128,"record_offset":896,"parent_record_offset":860,"ancestor_record_offset":536,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":1240,"parent_record_offset":1204,"ancestor_record_offset":536,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":536,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":128,"record_offset":1272,"parent_record_offset":1240,"ancestor_record_offset":536,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":1300,"parent_record_offset":1240,"ancestor_record_offset":536,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":1332,"parent_record_offset":1204,"ancestor_record_offset":536,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":128,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":536,"inlinee":13244,"source_line":223,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::movable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":128,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":536,"inlinee":13245,"source_line":121,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::modal_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 543
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33D3
// 首条位置: {"module":128,"inlinee":13267}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":128,"record_offset":60272,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":128,"record_offset":63060,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":62052,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":128,"record_offset":61992,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::separator
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":128,"record_offset":62584,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":7345,"source_line":1559,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::file_name
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":128,"record_offset":59484,"parent_record_offset":59316,"ancestor_record_offset":59316,"inlinee":7378,"source_line":651,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":59316,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":59316,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61692,"parent_record_offset":61660,"ancestor_record_offset":59316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61724,"parent_record_offset":61692,"ancestor_record_offset":59316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":59316,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61756,"parent_record_offset":61724,"ancestor_record_offset":59316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60052,"parent_record_offset":59896,"ancestor_record_offset":59316,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61660,"parent_record_offset":61580,"ancestor_record_offset":59316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":128,"record_offset":59632,"parent_record_offset":59548,"ancestor_record_offset":59316,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":128,"record_offset":60320,"parent_record_offset":60296,"ancestor_record_offset":59316,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59840,"parent_record_offset":59812,"ancestor_record_offset":59316,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61624,"parent_record_offset":61580,"ancestor_record_offset":59316,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61580,"parent_record_offset":61552,"ancestor_record_offset":59316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59896,"parent_record_offset":59868,"ancestor_record_offset":59316,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59868,"parent_record_offset":59840,"ancestor_record_offset":59316,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61552,"parent_record_offset":61524,"ancestor_record_offset":59316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59940,"parent_record_offset":59896,"ancestor_record_offset":59316,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59720,"parent_record_offset":59692,"ancestor_record_offset":59316,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59748,"parent_record_offset":59720,"ancestor_record_offset":59316,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59776,"parent_record_offset":59748,"ancestor_record_offset":59316,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59692,"parent_record_offset":59664,"ancestor_record_offset":59316,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":128,"record_offset":59664,"parent_record_offset":59632,"ancestor_record_offset":59316,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":62168,"parent_record_offset":62132,"ancestor_record_offset":59316,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":62132,"parent_record_offset":62104,"ancestor_record_offset":59316,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":128,"record_offset":59812,"parent_record_offset":59776,"ancestor_record_offset":59316,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":128,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59316,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":128,"record_offset":60012,"parent_record_offset":59988,"ancestor_record_offset":59316,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":128,"record_offset":59988,"parent_record_offset":59964,"ancestor_record_offset":59316,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":128,"record_offset":59516,"parent_record_offset":59484,"ancestor_record_offset":59316,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":128,"record_offset":60296,"parent_record_offset":60272,"ancestor_record_offset":59316,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":128,"record_offset":60180,"parent_record_offset":59776,"ancestor_record_offset":59316,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":60204,"parent_record_offset":60180,"ancestor_record_offset":59316,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":61496,"parent_record_offset":61148,"ancestor_record_offset":59316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":128,"record_offset":62200,"parent_record_offset":62168,"ancestor_record_offset":59316,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":62228,"parent_record_offset":62168,"ancestor_record_offset":59316,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":62260,"parent_record_offset":62132,"ancestor_record_offset":59316,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":128,"record_offset":62016,"parent_record_offset":61992,"ancestor_record_offset":59316,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::separator::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/separator.rs
//     evidence={"module":128,"record_offset":62608,"parent_record_offset":62584,"ancestor_record_offset":59316,"inlinee":7347,"source_line":22,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":128,"record_offset":59576,"parent_record_offset":59548,"ancestor_record_offset":59316,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":128,"record_offset":59600,"parent_record_offset":59576,"ancestor_record_offset":59316,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::file_name::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":128,"record_offset":59548,"parent_record_offset":59516,"ancestor_record_offset":59316,"inlinee":7302,"source_line":653,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x463D
// 首条位置: {"module":252,"inlinee":17981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":252,"record_offset":101544,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":252,"record_offset":101248,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":252,"record_offset":101884,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::clutter_audio::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/clutter_audio.rs
//     evidence={"module":252,"record_offset":101628,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":13265,"source_line":11,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":252,"record_offset":101800,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::selectable_label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":252,"record_offset":101656,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":16428,"source_line":1531,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":252,"record_offset":101220,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":10540,"source_line":100,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":252,"record_offset":102372,"parent_record_offset":101152,"ancestor_record_offset":101152,"inlinee":15769,"source_line":177,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":252,"record_offset":101592,"parent_record_offset":101568,"ancestor_record_offset":101152,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":252,"record_offset":103048,"parent_record_offset":103016,"ancestor_record_offset":101152,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":252,"record_offset":103096,"parent_record_offset":103072,"ancestor_record_offset":101152,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":252,"record_offset":103072,"parent_record_offset":103048,"ancestor_record_offset":101152,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":252,"record_offset":101992,"parent_record_offset":101956,"ancestor_record_offset":101152,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":252,"record_offset":101956,"parent_record_offset":101932,"ancestor_record_offset":101152,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":252,"record_offset":103016,"parent_record_offset":102972,"ancestor_record_offset":101152,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":252,"record_offset":102940,"parent_record_offset":102904,"ancestor_record_offset":101152,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":252,"record_offset":102972,"parent_record_offset":102940,"ancestor_record_offset":101152,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":252,"record_offset":102876,"parent_record_offset":102840,"ancestor_record_offset":101152,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":252,"record_offset":102840,"parent_record_offset":102816,"ancestor_record_offset":101152,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":252,"record_offset":102816,"parent_record_offset":102372,"ancestor_record_offset":101152,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":252,"record_offset":102904,"parent_record_offset":102840,"ancestor_record_offset":101152,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":252,"record_offset":101464,"parent_record_offset":101440,"ancestor_record_offset":101152,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":252,"record_offset":101708,"parent_record_offset":101680,"ancestor_record_offset":101152,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":252,"record_offset":102628,"parent_record_offset":102572,"ancestor_record_offset":101152,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":252,"record_offset":102540,"parent_record_offset":102516,"ancestor_record_offset":101152,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":252,"record_offset":101276,"parent_record_offset":101248,"ancestor_record_offset":101152,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":252,"record_offset":101392,"parent_record_offset":101304,"ancestor_record_offset":101152,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":252,"record_offset":101304,"parent_record_offset":101276,"ancestor_record_offset":101152,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":252,"record_offset":101416,"parent_record_offset":101392,"ancestor_record_offset":101152,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":252,"record_offset":102424,"parent_record_offset":102400,"ancestor_record_offset":101152,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":252,"record_offset":101568,"parent_record_offset":101544,"ancestor_record_offset":101152,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":252,"record_offset":101360,"parent_record_offset":101332,"ancestor_record_offset":101152,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":252,"record_offset":101908,"parent_record_offset":101884,"ancestor_record_offset":101152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":252,"record_offset":102024,"parent_record_offset":101992,"ancestor_record_offset":101152,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":252,"record_offset":103136,"parent_record_offset":102972,"ancestor_record_offset":101152,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":252,"record_offset":102052,"parent_record_offset":101992,"ancestor_record_offset":101152,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":252,"record_offset":102084,"parent_record_offset":101956,"ancestor_record_offset":101152,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::audio::clutter_audio::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/clutter_audio.rs
//     evidence={"module":252,"record_offset":101488,"parent_record_offset":101464,"ancestor_record_offset":101152,"inlinee":13265,"source_line":11,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":252,"record_offset":101848,"parent_record_offset":101824,"ancestor_record_offset":101152,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":252,"record_offset":101824,"parent_record_offset":101800,"ancestor_record_offset":101152,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":252,"record_offset":101756,"parent_record_offset":101732,"ancestor_record_offset":101152,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":252,"record_offset":101732,"parent_record_offset":101708,"ancestor_record_offset":101152,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::selected_label::SelectableLabel::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/selected_label.rs
//     evidence={"module":252,"record_offset":101680,"parent_record_offset":101656,"ancestor_record_offset":101152,"inlinee":15865,"source_line":31,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":252,"record_offset":101332,"parent_record_offset":101304,"ancestor_record_offset":101152,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":252,"record_offset":102480,"parent_record_offset":102452,"ancestor_record_offset":101152,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":252,"record_offset":102516,"parent_record_offset":102480,"ancestor_record_offset":101152,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":252,"record_offset":102660,"parent_record_offset":102480,"ancestor_record_offset":101152,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":252,"record_offset":102772,"parent_record_offset":102480,"ancestor_record_offset":101152,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":252,"record_offset":101440,"parent_record_offset":101416,"ancestor_record_offset":101152,"inlinee":13263,"source_line":605,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":252,"record_offset":102400,"parent_record_offset":102372,"ancestor_record_offset":101152,"inlinee":7375,"source_line":727,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":252,"record_offset":102452,"parent_record_offset":102424,"ancestor_record_offset":101152,"inlinee":7184,"source_line":729,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 605
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33CF
// 首条位置: {"module":128,"inlinee":13263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":252,"record_offset":101464,"parent_record_offset":101440,"ancestor_record_offset":101440,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::audio::clutter_audio::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/clutter_audio.rs
//     evidence={"module":252,"record_offset":101488,"parent_record_offset":101464,"ancestor_record_offset":101440,"inlinee":13265,"source_line":11,"archive":"_pdb/C/modules/00252.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::question
// 记录形式: compiled_procedure; instances=1
// 源码行: 635
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":28,"record_offset":20920,"rva":1384288,"va":5370093408}
// 编译实例 RVAs: 0x151f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12718606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e80e12c200","size":5,"soft_fail":false,"target_va":5382812128,"va":5370093517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5585120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e8e0385500","size":5,"soft_fail":false,"target_va":5375678688,"va":5370093563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1371361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e81f13ebff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370093548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140436220
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3031297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e801412e00","size":5,"soft_fail":false,"target_va":5373125152,"va":5370093850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054ca00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4172363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e84baa3f00","size":5,"soft_fail":false,"target_va":5374265856,"va":5370093488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140697f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_mod_editor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5529224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e8885e5400","size":5,"soft_fail":false,"target_va":5375622976,"va":5370093747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46526963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e8f3f1c502","size":5,"soft_fail":false,"target_va":5416620769,"va":5370093801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12678363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e8db74c100","size":5,"soft_fail":false,"target_va":5382771904,"va":5370093536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::ui::Ui::available_size_before_wrap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12309606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370093408,"raw_hex":"e866d4bb00","size":5,"soft_fail":false,"target_va":5382403248,"va":5370093637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":21144,"parent_record_offset":20920,"ancestor_record_offset":20920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":28,"record_offset":21060,"parent_record_offset":20920,"ancestor_record_offset":20920,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":21384,"parent_record_offset":20920,"ancestor_record_offset":20920,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":21088,"parent_record_offset":20920,"ancestor_record_offset":20920,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":21504,"parent_record_offset":21480,"ancestor_record_offset":20920,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":20920,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":21552,"parent_record_offset":21528,"ancestor_record_offset":20920,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":21480,"parent_record_offset":21448,"ancestor_record_offset":20920,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":21448,"parent_record_offset":21412,"ancestor_record_offset":20920,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":28,"record_offset":21412,"parent_record_offset":21384,"ancestor_record_offset":20920,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":21248,"parent_record_offset":21212,"ancestor_record_offset":20920,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":20920,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":21212,"parent_record_offset":21188,"ancestor_record_offset":20920,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":21164,"parent_record_offset":21144,"ancestor_record_offset":20920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":21280,"parent_record_offset":21248,"ancestor_record_offset":20920,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":21308,"parent_record_offset":21248,"ancestor_record_offset":20920,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":21340,"parent_record_offset":21212,"ancestor_record_offset":20920,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::Layout::prefer_right_to_left
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":21792,"parent_record_offset":21764,"ancestor_record_offset":20920,"inlinee":7296,"source_line":333,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::layout::impl$16::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/layout.rs
//     evidence={"module":28,"record_offset":21824,"parent_record_offset":21792,"ancestor_record_offset":20920,"inlinee":7298,"source_line":78,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::placer::Placer::prefer_right_to_left
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/placer.rs
//     evidence={"module":28,"record_offset":21764,"parent_record_offset":21604,"ancestor_record_offset":20920,"inlinee":7294,"source_line":47,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::horizontal_with_main_wrap_dyn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":21604,"parent_record_offset":21384,"ancestor_record_offset":20920,"inlinee":7291,"source_line":1964,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::ui::Ui::spacing
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":21652,"parent_record_offset":21604,"ancestor_record_offset":20920,"inlinee":7292,"source_line":187,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":28,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":20920,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] emath::align::impl$11::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/align.rs
//     evidence={"module":28,"record_offset":21852,"parent_record_offset":21792,"ancestor_record_offset":20920,"inlinee":7300,"source_line":6,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::question::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 640
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCA
// 首条位置: {"module":28,"inlinee":7370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":46128,"parent_record_offset":45892,"ancestor_record_offset":45892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":46040,"parent_record_offset":45892,"ancestor_record_offset":45892,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":28,"record_offset":45944,"parent_record_offset":45892,"ancestor_record_offset":45892,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":46244,"parent_record_offset":46208,"ancestor_record_offset":45892,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":28,"record_offset":46208,"parent_record_offset":46180,"ancestor_record_offset":45892,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":46004,"parent_record_offset":45972,"ancestor_record_offset":45892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":46276,"parent_record_offset":46244,"ancestor_record_offset":45892,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":46304,"parent_record_offset":46244,"ancestor_record_offset":45892,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":28,"record_offset":46336,"parent_record_offset":46208,"ancestor_record_offset":45892,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":28,"record_offset":46092,"parent_record_offset":46068,"ancestor_record_offset":45892,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":28,"record_offset":46068,"parent_record_offset":46040,"ancestor_record_offset":45892,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":28,"record_offset":45972,"parent_record_offset":45944,"ancestor_record_offset":45892,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::file_name
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 651
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CD2
// 首条位置: {"module":128,"inlinee":7378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":128,"record_offset":59516,"parent_record_offset":59484,"ancestor_record_offset":59484,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":61496,"parent_record_offset":61148,"ancestor_record_offset":61148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":59484,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":59484,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61692,"parent_record_offset":61660,"ancestor_record_offset":61148,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61724,"parent_record_offset":61692,"ancestor_record_offset":61148,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":59484,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61756,"parent_record_offset":61724,"ancestor_record_offset":61148,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60052,"parent_record_offset":59896,"ancestor_record_offset":59484,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":61660,"parent_record_offset":61580,"ancestor_record_offset":61148,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":128,"record_offset":59632,"parent_record_offset":59548,"ancestor_record_offset":59484,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59840,"parent_record_offset":59812,"ancestor_record_offset":59484,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61624,"parent_record_offset":61580,"ancestor_record_offset":61148,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61580,"parent_record_offset":61552,"ancestor_record_offset":61148,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59896,"parent_record_offset":59868,"ancestor_record_offset":59484,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59868,"parent_record_offset":59840,"ancestor_record_offset":59484,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":61552,"parent_record_offset":61524,"ancestor_record_offset":61148,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59940,"parent_record_offset":59896,"ancestor_record_offset":59484,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59720,"parent_record_offset":59692,"ancestor_record_offset":59484,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59748,"parent_record_offset":59720,"ancestor_record_offset":59484,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59776,"parent_record_offset":59748,"ancestor_record_offset":59484,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59692,"parent_record_offset":59664,"ancestor_record_offset":59484,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":128,"record_offset":59664,"parent_record_offset":59632,"ancestor_record_offset":59484,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":128,"record_offset":59812,"parent_record_offset":59776,"ancestor_record_offset":59484,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":128,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59484,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":128,"record_offset":60012,"parent_record_offset":59988,"ancestor_record_offset":59484,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":128,"record_offset":59988,"parent_record_offset":59964,"ancestor_record_offset":59484,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":128,"record_offset":60180,"parent_record_offset":59776,"ancestor_record_offset":59484,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":60204,"parent_record_offset":60180,"ancestor_record_offset":59484,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":61524,"parent_record_offset":61496,"ancestor_record_offset":61148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":128,"record_offset":59576,"parent_record_offset":59548,"ancestor_record_offset":59484,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":128,"record_offset":59600,"parent_record_offset":59576,"ancestor_record_offset":59484,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::file_name::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":128,"record_offset":59548,"parent_record_offset":59516,"ancestor_record_offset":59484,"inlinee":7302,"source_line":653,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::file_name::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 653
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C86
// 首条位置: {"module":28,"inlinee":7302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":128,"record_offset":59632,"parent_record_offset":59548,"ancestor_record_offset":59548,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":128,"record_offset":59576,"parent_record_offset":59548,"ancestor_record_offset":59548,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":59548,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":59548,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":59548,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":60052,"parent_record_offset":59896,"ancestor_record_offset":59548,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59840,"parent_record_offset":59812,"ancestor_record_offset":59548,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59896,"parent_record_offset":59868,"ancestor_record_offset":59548,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59868,"parent_record_offset":59840,"ancestor_record_offset":59548,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":59940,"parent_record_offset":59896,"ancestor_record_offset":59548,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59720,"parent_record_offset":59692,"ancestor_record_offset":59548,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59748,"parent_record_offset":59720,"ancestor_record_offset":59548,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59776,"parent_record_offset":59748,"ancestor_record_offset":59548,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":128,"record_offset":59692,"parent_record_offset":59664,"ancestor_record_offset":59548,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":128,"record_offset":59664,"parent_record_offset":59632,"ancestor_record_offset":59548,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":128,"record_offset":59812,"parent_record_offset":59776,"ancestor_record_offset":59548,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":128,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59548,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":128,"record_offset":60012,"parent_record_offset":59988,"ancestor_record_offset":59548,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":128,"record_offset":59988,"parent_record_offset":59964,"ancestor_record_offset":59548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":128,"record_offset":60180,"parent_record_offset":59776,"ancestor_record_offset":59548,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":60204,"parent_record_offset":60180,"ancestor_record_offset":59548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":128,"record_offset":59600,"parent_record_offset":59576,"ancestor_record_offset":59548,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::item_statuses
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 659
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C1D
// 首条位置: {"module":28,"inlinee":7197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":980,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":896,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":812,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":764,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":28,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":764,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":764,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":28,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":764,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::item_statuses::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 667
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42B8
// 首条位置: {"module":228,"inlinee":17080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":41024,"parent_record_offset":40932,"ancestor_record_offset":40932,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":228,"record_offset":43824,"parent_record_offset":40932,"ancestor_record_offset":40932,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":41640,"parent_record_offset":40932,"ancestor_record_offset":40932,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":228,"record_offset":41108,"parent_record_offset":40932,"ancestor_record_offset":40932,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41940,"parent_record_offset":41916,"ancestor_record_offset":40932,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41964,"parent_record_offset":41940,"ancestor_record_offset":40932,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41988,"parent_record_offset":41964,"ancestor_record_offset":40932,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41916,"parent_record_offset":41860,"ancestor_record_offset":40932,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":41072,"parent_record_offset":41048,"ancestor_record_offset":40932,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44152,"parent_record_offset":44120,"ancestor_record_offset":40932,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":43992,"parent_record_offset":43964,"ancestor_record_offset":40932,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41888,"parent_record_offset":41860,"ancestor_record_offset":40932,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41860,"parent_record_offset":41836,"ancestor_record_offset":40932,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44052,"parent_record_offset":44020,"ancestor_record_offset":40932,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44200,"parent_record_offset":44176,"ancestor_record_offset":40932,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44176,"parent_record_offset":44152,"ancestor_record_offset":40932,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44020,"parent_record_offset":43992,"ancestor_record_offset":40932,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41836,"parent_record_offset":41812,"ancestor_record_offset":40932,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43920,"parent_record_offset":43888,"ancestor_record_offset":40932,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":44120,"parent_record_offset":43920,"ancestor_record_offset":40932,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43856,"parent_record_offset":43824,"ancestor_record_offset":40932,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":44240,"parent_record_offset":43920,"ancestor_record_offset":40932,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43964,"parent_record_offset":43920,"ancestor_record_offset":40932,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":41668,"parent_record_offset":41640,"ancestor_record_offset":40932,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":228,"record_offset":43888,"parent_record_offset":43856,"ancestor_record_offset":40932,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41248,"parent_record_offset":41196,"ancestor_record_offset":40932,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41448,"parent_record_offset":41296,"ancestor_record_offset":40932,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":228,"record_offset":41560,"parent_record_offset":41532,"ancestor_record_offset":40932,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":228,"record_offset":41532,"parent_record_offset":41296,"ancestor_record_offset":40932,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":228,"record_offset":43664,"parent_record_offset":43628,"ancestor_record_offset":40932,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":228,"record_offset":44076,"parent_record_offset":44052,"ancestor_record_offset":40932,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":40932,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41220,"parent_record_offset":41196,"ancestor_record_offset":40932,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41048,"parent_record_offset":41024,"ancestor_record_offset":40932,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":41692,"parent_record_offset":41668,"ancestor_record_offset":40932,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":228,"record_offset":41160,"parent_record_offset":41136,"ancestor_record_offset":40932,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":40932,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41496,"parent_record_offset":41472,"ancestor_record_offset":40932,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":228,"record_offset":41136,"parent_record_offset":41108,"ancestor_record_offset":40932,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41364,"parent_record_offset":41296,"ancestor_record_offset":40932,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41336,"parent_record_offset":41296,"ancestor_record_offset":40932,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41296,"parent_record_offset":41272,"ancestor_record_offset":40932,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41272,"parent_record_offset":41248,"ancestor_record_offset":40932,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::item_statuses::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":228,"record_offset":41196,"parent_record_offset":41108,"ancestor_record_offset":40932,"inlinee":17082,"source_line":677,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::item_statuses::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 677
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42BA
// 首条位置: {"module":228,"inlinee":17082}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41248,"parent_record_offset":41196,"ancestor_record_offset":41196,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41220,"parent_record_offset":41196,"ancestor_record_offset":41196,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41448,"parent_record_offset":41296,"ancestor_record_offset":41196,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":228,"record_offset":41560,"parent_record_offset":41532,"ancestor_record_offset":41196,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":228,"record_offset":41532,"parent_record_offset":41296,"ancestor_record_offset":41196,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":41196,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":41196,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41496,"parent_record_offset":41472,"ancestor_record_offset":41196,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41364,"parent_record_offset":41296,"ancestor_record_offset":41196,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41336,"parent_record_offset":41296,"ancestor_record_offset":41196,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41296,"parent_record_offset":41272,"ancestor_record_offset":41196,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41272,"parent_record_offset":41248,"ancestor_record_offset":41196,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::item_statuses::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42B7
// 首条位置: {"module":228,"inlinee":17079}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":46936,"parent_record_offset":40864,"ancestor_record_offset":40864,"inlinee":6651,"source_line":2145,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":228,"record_offset":45916,"parent_record_offset":40864,"ancestor_record_offset":40864,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":228,"record_offset":44892,"parent_record_offset":40864,"ancestor_record_offset":40864,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::item_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":228,"record_offset":44572,"parent_record_offset":40864,"ancestor_record_offset":40864,"inlinee":7232,"source_line":696,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::item_statuses::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":228,"record_offset":40932,"parent_record_offset":40864,"ancestor_record_offset":40864,"inlinee":17080,"source_line":667,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41940,"parent_record_offset":41916,"ancestor_record_offset":40864,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41964,"parent_record_offset":41940,"ancestor_record_offset":40864,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41988,"parent_record_offset":41964,"ancestor_record_offset":40864,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":41916,"parent_record_offset":41860,"ancestor_record_offset":40864,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":41024,"parent_record_offset":40932,"ancestor_record_offset":40864,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":41072,"parent_record_offset":41048,"ancestor_record_offset":40864,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44152,"parent_record_offset":44120,"ancestor_record_offset":40864,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":43992,"parent_record_offset":43964,"ancestor_record_offset":40864,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41888,"parent_record_offset":41860,"ancestor_record_offset":40864,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41860,"parent_record_offset":41836,"ancestor_record_offset":40864,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44052,"parent_record_offset":44020,"ancestor_record_offset":40864,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44200,"parent_record_offset":44176,"ancestor_record_offset":40864,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44176,"parent_record_offset":44152,"ancestor_record_offset":40864,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":44020,"parent_record_offset":43992,"ancestor_record_offset":40864,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":41836,"parent_record_offset":41812,"ancestor_record_offset":40864,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":228,"record_offset":43824,"parent_record_offset":40932,"ancestor_record_offset":40864,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43920,"parent_record_offset":43888,"ancestor_record_offset":40864,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":44120,"parent_record_offset":43920,"ancestor_record_offset":40864,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43856,"parent_record_offset":43824,"ancestor_record_offset":40864,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":44240,"parent_record_offset":43920,"ancestor_record_offset":40864,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":43964,"parent_record_offset":43920,"ancestor_record_offset":40864,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":41668,"parent_record_offset":41640,"ancestor_record_offset":40864,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":228,"record_offset":43888,"parent_record_offset":43856,"ancestor_record_offset":40864,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41248,"parent_record_offset":41196,"ancestor_record_offset":40864,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":41448,"parent_record_offset":41296,"ancestor_record_offset":40864,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":228,"record_offset":45184,"parent_record_offset":45160,"ancestor_record_offset":40864,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":228,"record_offset":45268,"parent_record_offset":45244,"ancestor_record_offset":40864,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":228,"record_offset":45740,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":228,"record_offset":41560,"parent_record_offset":41532,"ancestor_record_offset":40864,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":228,"record_offset":41532,"parent_record_offset":41296,"ancestor_record_offset":40864,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":228,"record_offset":43664,"parent_record_offset":43628,"ancestor_record_offset":40864,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":228,"record_offset":44076,"parent_record_offset":44052,"ancestor_record_offset":40864,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":228,"record_offset":45416,"parent_record_offset":45388,"ancestor_record_offset":40864,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":228,"record_offset":45388,"parent_record_offset":45360,"ancestor_record_offset":40864,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":228,"record_offset":46992,"parent_record_offset":46964,"ancestor_record_offset":40864,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":40864,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41220,"parent_record_offset":41196,"ancestor_record_offset":40864,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":46964,"parent_record_offset":46936,"ancestor_record_offset":40864,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":41048,"parent_record_offset":41024,"ancestor_record_offset":40864,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":44860,"parent_record_offset":44572,"ancestor_record_offset":40864,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":45208,"parent_record_offset":45184,"ancestor_record_offset":40864,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":41640,"parent_record_offset":40932,"ancestor_record_offset":40864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":228,"record_offset":45532,"parent_record_offset":45508,"ancestor_record_offset":40864,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":228,"record_offset":41160,"parent_record_offset":41136,"ancestor_record_offset":40864,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":40864,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":41496,"parent_record_offset":41472,"ancestor_record_offset":40864,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":228,"record_offset":44700,"parent_record_offset":44672,"ancestor_record_offset":40864,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":228,"record_offset":41108,"parent_record_offset":40932,"ancestor_record_offset":40864,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":228,"record_offset":41136,"parent_record_offset":41108,"ancestor_record_offset":40864,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":228,"record_offset":44616,"parent_record_offset":44572,"ancestor_record_offset":40864,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":228,"record_offset":44672,"parent_record_offset":44644,"ancestor_record_offset":40864,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":228,"record_offset":44644,"parent_record_offset":44616,"ancestor_record_offset":40864,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_diagnostics::impl$0::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_diagnostics.rs
//     evidence={"module":228,"record_offset":47020,"parent_record_offset":46992,"ancestor_record_offset":40864,"inlinee":16661,"source_line":12,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":228,"record_offset":45640,"parent_record_offset":45616,"ancestor_record_offset":40864,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":228,"record_offset":45328,"parent_record_offset":45300,"ancestor_record_offset":40864,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":228,"record_offset":45360,"parent_record_offset":45328,"ancestor_record_offset":40864,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":228,"record_offset":45300,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":228,"record_offset":45160,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":228,"record_offset":45768,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":228,"record_offset":45244,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":228,"record_offset":45104,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":228,"record_offset":45592,"parent_record_offset":45568,"ancestor_record_offset":40864,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":228,"record_offset":44940,"parent_record_offset":44916,"ancestor_record_offset":40864,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":228,"record_offset":45948,"parent_record_offset":45916,"ancestor_record_offset":40864,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":228,"record_offset":45568,"parent_record_offset":45460,"ancestor_record_offset":40864,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":228,"record_offset":44972,"parent_record_offset":44940,"ancestor_record_offset":40864,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45508,"parent_record_offset":45484,"ancestor_record_offset":40864,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45860,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":40864,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45024,"parent_record_offset":45000,"ancestor_record_offset":40864,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45000,"parent_record_offset":44940,"ancestor_record_offset":40864,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45048,"parent_record_offset":45024,"ancestor_record_offset":40864,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45132,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":228,"record_offset":45460,"parent_record_offset":45048,"ancestor_record_offset":40864,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":228,"record_offset":44916,"parent_record_offset":44892,"ancestor_record_offset":40864,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41364,"parent_record_offset":41296,"ancestor_record_offset":40864,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41336,"parent_record_offset":41296,"ancestor_record_offset":40864,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41296,"parent_record_offset":41272,"ancestor_record_offset":40864,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":228,"record_offset":41272,"parent_record_offset":41248,"ancestor_record_offset":40864,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::item_statuses::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":228,"record_offset":41196,"parent_record_offset":41108,"ancestor_record_offset":40864,"inlinee":17082,"source_line":677,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":228,"record_offset":45664,"parent_record_offset":45640,"ancestor_record_offset":40864,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::item_id
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C40
// 首条位置: {"module":28,"inlinee":7232}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":8300,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":8332,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":28,"record_offset":8088,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":8048,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":8048,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":8048,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":8048,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8144,"parent_record_offset":8116,"ancestor_record_offset":8048,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":8048,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::log_new_failures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C39
// 首条位置: {"module":28,"inlinee":7225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":10540,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7512,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7680,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":10020,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":10076,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":9812,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":28,"record_offset":10624,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7920,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":28,"record_offset":7540,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":8480,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":8816,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":10200,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":28,"record_offset":8364,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_mod_editor::item_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":28,"record_offset":8048,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":7232,"source_line":696,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":7412,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":7412,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":7412,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":28,"record_offset":8676,"parent_record_offset":8620,"ancestor_record_offset":7412,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":28,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":7412,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7796,"parent_record_offset":7764,"ancestor_record_offset":7412,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":8648,"parent_record_offset":8620,"ancestor_record_offset":7412,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":8620,"parent_record_offset":8592,"ancestor_record_offset":7412,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":7412,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":7412,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":28,"record_offset":8592,"parent_record_offset":8564,"ancestor_record_offset":7412,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":28,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":7412,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7764,"parent_record_offset":7716,"ancestor_record_offset":7412,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":28,"record_offset":7716,"parent_record_offset":7680,"ancestor_record_offset":7412,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":11932,"parent_record_offset":11244,"ancestor_record_offset":7412,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":7412,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":7412,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":28,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":7412,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":28,"record_offset":9912,"parent_record_offset":9884,"ancestor_record_offset":7412,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":7412,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":7412,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":28,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":7412,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":28,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":7412,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":28,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":7412,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":28,"record_offset":9640,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7412,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7412,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":28,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":7412,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":28,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":7412,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":28,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":7412,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":28,"record_offset":12280,"parent_record_offset":12252,"ancestor_record_offset":7412,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":28,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":7412,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":28,"record_offset":9288,"parent_record_offset":9260,"ancestor_record_offset":7412,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":8300,"parent_record_offset":8048,"ancestor_record_offset":7412,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":7412,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":8332,"parent_record_offset":8048,"ancestor_record_offset":7412,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":28,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":7412,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":28,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":7412,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":7412,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":8508,"parent_record_offset":8480,"ancestor_record_offset":7412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":7412,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":7412,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":28,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":7412,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":7412,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":10140,"parent_record_offset":10076,"ancestor_record_offset":7412,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":28,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":7412,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":7412,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":28,"record_offset":7884,"parent_record_offset":7716,"ancestor_record_offset":7412,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":7412,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":28,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":7412,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":28,"record_offset":8172,"parent_record_offset":8144,"ancestor_record_offset":7412,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":28,"record_offset":7984,"parent_record_offset":7948,"ancestor_record_offset":7412,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":28,"record_offset":8088,"parent_record_offset":8048,"ancestor_record_offset":7412,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8144,"parent_record_offset":8116,"ancestor_record_offset":7412,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":28,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7412,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":28,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":7412,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":7412,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9260,"parent_record_offset":9228,"ancestor_record_offset":7412,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":28,"record_offset":9200,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9060,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9668,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":28,"record_offset":9144,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":28,"record_offset":9004,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":28,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":7412,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":7412,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7412,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7412,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":9468,"parent_record_offset":9360,"ancestor_record_offset":7412,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":28,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":7412,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":7412,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9760,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":7412,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":7412,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8900,"parent_record_offset":8840,"ancestor_record_offset":7412,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":8948,"parent_record_offset":8924,"ancestor_record_offset":7412,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9032,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":28,"record_offset":9360,"parent_record_offset":8948,"ancestor_record_offset":7412,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":28,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":7412,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/00028.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::preview_position
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 727
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCF
// 首条位置: {"module":192,"inlinee":7375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":192,"record_offset":7284,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7100,"parent_record_offset":7044,"ancestor_record_offset":6864,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6864,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":192,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":6864,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":6988,"parent_record_offset":6952,"ancestor_record_offset":6864,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":7132,"parent_record_offset":6952,"ancestor_record_offset":6864,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":192,"record_offset":7244,"parent_record_offset":6952,"ancestor_record_offset":6864,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_mod_editor::preview_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_mod_editor.rs
//     evidence={"module":192,"record_offset":6924,"parent_record_offset":6892,"ancestor_record_offset":6864,"inlinee":7184,"source_line":729,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_mod_editor::preview_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 729
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C10
// 首条位置: {"module":28,"inlinee":7184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":192,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":6924,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7100,"parent_record_offset":7044,"ancestor_record_offset":6924,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":192,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6924,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":6988,"parent_record_offset":6952,"ancestor_record_offset":6924,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":192,"record_offset":7132,"parent_record_offset":6952,"ancestor_record_offset":6924,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00192.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":192,"record_offset":7244,"parent_record_offset":6952,"ancestor_record_offset":6924,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00192.jsonl.gz"}
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
// 完整结构及原始类型记录: gui_mod_editor.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_mod_editor.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_mod_editor.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<ecolor::color32::Color32> > (*)(usize,ref$<egui::style::Style>)
// 候选参数类型: ["usize", "ref$<egui::style::Style>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>", "country_core::systems::steam::SteamOverlayOpenResources", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor
// 信息 ID: f4f33339cb90585b14cfea2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5849014cc0833440d1efa177: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >)> > @ 0x27ddf1 -> 0x14fd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >)> @ 0x404b73 -> 0x14fd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$0
// 信息 ID: 32c4059158707a6f2ce83006
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::do_gui_mod_editor", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- tiny_glade::gui::gui_mod_editor::modal_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- tiny_glade::gui::gui_mod_editor::editing_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0 <- egui::ui::Ui::horizontal_wrapped<tuple$<>,tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=192, inline_record=6836; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=252, inline_record=102372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1
// 信息 ID: dad69af8a2029a57f6dc0810
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_mod_editor::do_gui_mod_editor", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=29388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure$0
// 信息 ID: 79d00b22a9e150d1cd060052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_mod_editor::do_gui_mod_editor", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=29524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure$1
// 信息 ID: 78e281153de73703591c4e56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_mod_editor::do_gui_mod_editor", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=50848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editing_view::closure$0
// 信息 ID: 09b49d127b968eb506cf226a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::editing_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::editing_view::closure_env$0 <- core::ops::function::FnOnce::call_once<egui::containers::scroll_area::impl$5::show::closure_env$0<tuple$<>,tiny_glade::gui::gui_mod_editor::editing_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0> >,tuple$<ref_mut$<egui::ui::Ui>,emath::rect::Rect> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<egui::containers::scroll_area::impl$5::show::closure_env$0<tuple$<>,tiny_glade::gui::gui_mod_editor::editing_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0> >,tuple$<ref_mut$<egui::ui::Ui>,emath::rect::Rect> >; module=253, inline_record=5896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0
// 信息 ID: bd0f3bd8f8ba81edc98ce73f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::editing_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=192, inline_record=8728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure$0
// 信息 ID: 7540de959b3dbee3d73956c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::editing_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=208, inline_record=24588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure$1
// 信息 ID: 7395b520a2b275599fb1fd06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::editing_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=192, inline_record=5164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editing_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>
// 信息 ID: f7f58d2953d2918273193f81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::gui::gui_mod_editor::editing_view -> ["tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x152c59 -> 0x4027f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::editor_style
// 信息 ID: b7456b64c123e2e7c1a663db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor; module=28, inline_record=15996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::file_name
// 信息 ID: 741d576a0ccc32166792c6b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=59484; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=60356; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=61148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::file_name::closure$0
// 信息 ID: a09ef24a7def1fa829d00e19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::file_name", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=59548; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=60432; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=61212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::format_size
// 信息 ID: 10e2289e8b53bcad8d03d781
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=39380; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=41336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::item_id
// 信息 ID: 6060a911283c9eb3429fcde6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor; module=28, inline_record=8048; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; module=228, inline_record=44572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::item_statuses
// 信息 ID: 81b888558a903bd8f71d5233
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor; module=28, inline_record=764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::item_statuses::closure$0
// 信息 ID: 29b2d7367c9a1c5b8ccbc81f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::item_statuses", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; module=228, inline_record=40932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::item_statuses::closure$0::closure$0
// 信息 ID: 0245cef03df4d5a150d7ae8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::item_statuses", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; module=228, inline_record=41196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::item_statuses::closure$1
// 信息 ID: ab758e3c8a44214ce511972d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_mod_editor::item_statuses", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1 <- core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1 <- core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >; module=228, inline_record=40864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::log_new_failures
// 信息 ID: 1db83b868e48bbc6d1eb807d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor; module=28, inline_record=7412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::log_view
// 信息 ID: b96cd6d2c047a8755b9f38c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=44864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::log_view::closure$0
// 信息 ID: 7443c33a29aad99e64551d3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::log_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::log_view::closure_env$0 <- core::ops::function::FnOnce::call_once<egui::containers::scroll_area::impl$5::show::closure_env$0<tuple$<>,tiny_glade::gui::gui_mod_editor::log_view::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui>,emath::rect::Rect> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<egui::containers::scroll_area::impl$5::show::closure_env$0<tuple$<>,tiny_glade::gui::gui_mod_editor::log_view::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui>,emath::rect::Rect> >; module=253, inline_record=3244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::modal_view::closure$0
// 信息 ID: 65204b67fc8c50e88652355a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::modal_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=128, inline_record=59316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure$0
// 信息 ID: f0af8d320baa77e60e6ed10b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::modal_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0 <- egui::ui::Ui::horizontal_wrapped<tuple$<>,tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=252, inline_record=101152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure$0::closure$0
// 信息 ID: 3ed0dbc0733761207f424569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::modal_view", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=252, inline_record=101440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::modal_view<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>
// 信息 ID: de661d6a4eaaa5da8e3517ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::gui::gui_mod_editor::modal_view -> ["tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cee -> 0x402450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::new_mod_view
// 信息 ID: dc69da6fb4128ca3c692c2d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=31032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::new_mod_view::closure$0
// 信息 ID: 33df24dab393163deed89684
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::new_mod_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::new_mod_view::closure_env$0 <- egui::context::Context::write<bool,egui::context::impl$13::input::closure_env$0<bool,tiny_glade::gui::gui_mod_editor::new_mod_view::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] egui::context::Context::write<bool,egui::context::impl$13::input::closure_env$0<bool,tiny_glade::gui::gui_mod_editor::new_mod_view::closure_env$0> >; module=149, inline_record=45680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::preview_position
// 信息 ID: 827e6a917dd7990972f1df7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=192, inline_record=6864; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=252, inline_record=102400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::preview_position::closure$0
// 信息 ID: d7cc8681a5a144daed1695e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::preview_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::editing_view::closure$0::closure$0::closure_env$1<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=192, inline_record=6924; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure$0::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > >; module=252, inline_record=102452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::publish_view
// 信息 ID: 8a55ef0c4da8a8ac531be6ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=34224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::publish_view::closure$0
// 信息 ID: 54be2060939faa405d6fde14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::publish_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::publish_view::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::publish_view::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::publish_view::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=48044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::publish_view::closure$0::closure$0
// 信息 ID: 8df2e6cd851f089a304d534c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::publish_view", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::publish_view::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=48124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::publish_view::closure$1
// 信息 ID: 1a89bf0d66ed9a426661ea1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_mod_editor::publish_view", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::publish_view::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::publish_view::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::publish_view::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=27796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::question
// 信息 ID: a4cf1dcdcfd6b01974ad338c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x405429 -> 0x151f60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::modal_view::closure_env$0<tiny_glade::gui::gui_mod_editor::do_gui_mod_editor::closure_env$0>,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x405152 -> 0x151f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::question::closure$0
// 信息 ID: 8b502a827dbc73c85f49765b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_mod_editor::question", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_mod_editor::question::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::question::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_mod_editor::question::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=28, inline_record=45892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_mod_editor::register_fonts
// 信息 ID: a66f5501f25135d8180b8a22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor; module=28, inline_record=1012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
