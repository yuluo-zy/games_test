// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/gui/gui_error_log.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 517f1d9e7f2ba2a6e13135cd8130a43908ef622535379048dddbc6587dbe63bd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_error_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 56, 64
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":64,"record_offset":25568,"rva":1442832,"va":5370151952}
// 编译实例 RVAs: 0x160410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140053a80
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1100855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e8c933efff","size":5,"soft_fail":false,"target_va":5369051776,"va":5370152626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140053bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1100048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e8f036efff","size":5,"soft_fail":false,"target_va":5369052080,"va":5370152123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140053ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1099596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e8b438efff","size":5,"soft_fail":false,"target_va":5369052896,"va":5370152487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4929052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e81c364b00","size":5,"soft_fail":false,"target_va":5375081488,"va":5370152431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5173544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e828f14e00","size":5,"soft_fail":false,"target_va":5375325712,"va":5370152163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5363643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e8bbd75100","size":5,"soft_fail":false,"target_va":5375516032,"va":5370152384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41695012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e824377c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370152151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41388432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e890897702","size":5,"soft_fail":false,"target_va":5411540992,"va":5370152555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11406114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370151952,"raw_hex":"e8220bae00","size":5,"soft_fail":false,"target_va":5381558640,"va":5370152521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":64,"record_offset":25720,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":9966,"source_line":135,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":64,"record_offset":26392,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":64,"record_offset":26080,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":9972,"source_line":239,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::collapsible
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":64,"record_offset":26012,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":9970,"source_line":307,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::frame
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":64,"record_offset":26044,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":9971,"source_line":125,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":64,"record_offset":26172,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":9371,"source_line":365,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":26288,"parent_record_offset":26264,"ancestor_record_offset":25568,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":26312,"parent_record_offset":26288,"ancestor_record_offset":25568,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":26228,"parent_record_offset":26200,"ancestor_record_offset":25568,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":26264,"parent_record_offset":26228,"ancestor_record_offset":25568,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":64,"record_offset":26200,"parent_record_offset":26172,"ancestor_record_offset":25568,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":64,"record_offset":26556,"parent_record_offset":26520,"ancestor_record_offset":25568,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":64,"record_offset":26520,"parent_record_offset":26492,"ancestor_record_offset":25568,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::count::to_usize::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":64,"record_offset":25860,"parent_record_offset":25840,"ancestor_record_offset":25568,"inlinee":9968,"source_line":138,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":64,"record_offset":25792,"parent_record_offset":25768,"ancestor_record_offset":25568,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":64,"record_offset":25840,"parent_record_offset":25816,"ancestor_record_offset":25568,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":64,"record_offset":25768,"parent_record_offset":25744,"ancestor_record_offset":25568,"inlinee":9239,"source_line":49,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":64,"record_offset":25960,"parent_record_offset":25840,"ancestor_record_offset":25568,"inlinee":9245,"source_line":53,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":64,"record_offset":25744,"parent_record_offset":25720,"ancestor_record_offset":25568,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":25916,"parent_record_offset":25888,"ancestor_record_offset":25568,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":64,"record_offset":26416,"parent_record_offset":26392,"ancestor_record_offset":25568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":64,"record_offset":26588,"parent_record_offset":26556,"ancestor_record_offset":25568,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":64,"record_offset":26340,"parent_record_offset":26312,"ancestor_record_offset":25568,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":64,"record_offset":25816,"parent_record_offset":25792,"ancestor_record_offset":25568,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":64,"record_offset":26616,"parent_record_offset":26556,"ancestor_record_offset":25568,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":64,"record_offset":26648,"parent_record_offset":26520,"ancestor_record_offset":25568,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":64,"record_offset":26112,"parent_record_offset":26080,"ancestor_record_offset":25568,"inlinee":9973,"source_line":223,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::movable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":64,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":25568,"inlinee":9974,"source_line":121,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":64,"record_offset":25888,"parent_record_offset":25860,"ancestor_record_offset":25568,"inlinee":9969,"source_line":12,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26F1
// 首条位置: {"module":64,"inlinee":9969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":25916,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26C5
// 首条位置: {"module":64,"inlinee":9925}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":64,"record_offset":4004,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":64,"record_offset":4060,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":5084,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":5112,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":64,"record_offset":5236,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":64,"record_offset":5180,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7571,"source_line":1298,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":64,"record_offset":5148,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":9927,"source_line":254,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":64,"record_offset":5028,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7777,"source_line":167,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":64,"record_offset":4304,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":6308,"source_line":79,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":3928,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":3928,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":3928,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":4836,"parent_record_offset":4660,"ancestor_record_offset":3928,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":64,"record_offset":5744,"parent_record_offset":5688,"ancestor_record_offset":3928,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":3928,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":5716,"parent_record_offset":5688,"ancestor_record_offset":3928,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":3928,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":4660,"parent_record_offset":4628,"ancestor_record_offset":3928,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":3928,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":3928,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":64,"record_offset":4696,"parent_record_offset":4660,"ancestor_record_offset":3928,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":64,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":3928,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":64,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":3928,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":64,"record_offset":4452,"parent_record_offset":4424,"ancestor_record_offset":3928,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":64,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":3928,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":64,"record_offset":4424,"parent_record_offset":4396,"ancestor_record_offset":3928,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":64,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":3928,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":64,"record_offset":4368,"parent_record_offset":4340,"ancestor_record_offset":3928,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":64,"record_offset":5344,"parent_record_offset":5308,"ancestor_record_offset":3928,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":64,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":3928,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":64,"record_offset":4572,"parent_record_offset":4536,"ancestor_record_offset":3928,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":64,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3928,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":64,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":3928,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":64,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":3928,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":64,"record_offset":4340,"parent_record_offset":4304,"ancestor_record_offset":3928,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":64,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3928,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":64,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":3928,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":64,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":3928,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":64,"record_offset":4212,"parent_record_offset":4088,"ancestor_record_offset":3928,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3928,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":64,"record_offset":4936,"parent_record_offset":4536,"ancestor_record_offset":3928,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":64,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":3928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":64,"record_offset":5376,"parent_record_offset":5344,"ancestor_record_offset":3928,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":64,"record_offset":4172,"parent_record_offset":4124,"ancestor_record_offset":3928,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":64,"record_offset":4088,"parent_record_offset":4060,"ancestor_record_offset":3928,"inlinee":9926,"source_line":296,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":64,"record_offset":4124,"parent_record_offset":4088,"ancestor_record_offset":3928,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":64,"record_offset":5404,"parent_record_offset":5344,"ancestor_record_offset":3928,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":64,"record_offset":5436,"parent_record_offset":5308,"ancestor_record_offset":3928,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text_style
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":64,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":3928,"inlinee":7778,"source_line":153,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":64,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":3928,"inlinee":7573,"source_line":32,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":64,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3928,"inlinee":9393,"source_line":27,"archive":"_pdb/B/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x24B1
// 首条位置: {"module":56,"inlinee":9393}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":64,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00064.jsonl.gz"}
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
// 完整结构及原始类型记录: gui_error_log.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_error_log.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_error_log.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log
// 信息 ID: 66da9a00d4fb6d10d76e219c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 11c77c23cea2fdeb0f545043: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x14c9e1 -> 0x160410 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x4433bc -> 0x160410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 信息 ID: d162f4792cc71f0f222ba23f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_error_log::do_gui_error_log; module=64, inline_record=25888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 信息 ID: 2886c01b90e9d72d2264cd72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=64, inline_record=3928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 信息 ID: f97c0f4ec065bd1d9bb1f221
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=64, inline_record=4236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
