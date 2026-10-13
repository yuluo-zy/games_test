// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/gui/gui_error_log.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 517f1d9e7f2ba2a6e13135cd8130a43908ef622535379048dddbc6587dbe63bd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_error_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 12, 255
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":255,"record_offset":10620,"rva":3947504,"va":5372656624}
// 编译实例 RVAs: 0x3c3bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024d80
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3797271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e8e90ec6ff","size":5,"soft_fail":false,"target_va":5368860032,"va":5372657298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019f810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2245786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e866bbddff","size":5,"soft_fail":false,"target_va":5370411024,"va":5372656805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019fa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2245590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e82abcddff","size":5,"soft_fail":false,"target_va":5370411584,"va":5372657169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t92578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e8a2690100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372657113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040de80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t303534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e8aea10400","size":5,"soft_fail":false,"target_va":5372960384,"va":5372656845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t492769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e8e1840700","size":5,"soft_fail":false,"target_va":5373149840,"va":5372657066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30936522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e8ca0dd801","size":5,"soft_fail":false,"target_va":5403593360,"va":5372656833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30320902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e806a9ce01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372657237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1004776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372656624,"raw_hex":"e818abf0ff","size":5,"soft_fail":false,"target_va":5371652432,"va":5372657203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":255,"record_offset":10772,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":13757,"source_line":133,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":11472,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":255,"record_offset":11132,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":13767,"source_line":239,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::collapsible
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":255,"record_offset":11064,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":13765,"source_line":307,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::frame
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":255,"record_offset":11096,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":13766,"source_line":125,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":255,"record_offset":11224,"parent_record_offset":10620,"ancestor_record_offset":10620,"inlinee":11106,"source_line":365,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":10620,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":10620,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":10620,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":11316,"parent_record_offset":11280,"ancestor_record_offset":10620,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":255,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":10620,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":255,"record_offset":11636,"parent_record_offset":11600,"ancestor_record_offset":10620,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":255,"record_offset":11600,"parent_record_offset":11572,"ancestor_record_offset":10620,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::count::to_usize::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":255,"record_offset":10912,"parent_record_offset":10892,"ancestor_record_offset":10620,"inlinee":13761,"source_line":136,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":255,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10620,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":255,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":10620,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":255,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10620,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":255,"record_offset":11012,"parent_record_offset":10892,"ancestor_record_offset":10620,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":255,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":10620,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10620,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":11440,"parent_record_offset":11224,"ancestor_record_offset":10620,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":255,"record_offset":11668,"parent_record_offset":11636,"ancestor_record_offset":10620,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":10620,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":255,"record_offset":10868,"parent_record_offset":10844,"ancestor_record_offset":10620,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":255,"record_offset":11696,"parent_record_offset":11636,"ancestor_record_offset":10620,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":255,"record_offset":11728,"parent_record_offset":11600,"ancestor_record_offset":10620,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":255,"record_offset":11164,"parent_record_offset":11132,"ancestor_record_offset":10620,"inlinee":13768,"source_line":223,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::area::Area::movable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/area.rs
//     evidence={"module":255,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":10620,"inlinee":13769,"source_line":121,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":255,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10620,"inlinee":13762,"source_line":12,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35C2
// 首条位置: {"module":255,"inlinee":13762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10940,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35B2
// 首条位置: {"module":255,"inlinee":13746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":255,"record_offset":5120,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":255,"record_offset":5176,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":6132,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":6160,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":6284,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":255,"record_offset":6228,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5804,"source_line":1298,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":255,"record_offset":6196,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":13748,"source_line":254,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":255,"record_offset":6076,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":6634,"source_line":167,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":255,"record_offset":5464,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5819,"source_line":79,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":5044,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":5044,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":5044,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":5824,"parent_record_offset":5788,"ancestor_record_offset":5044,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":255,"record_offset":6772,"parent_record_offset":6716,"ancestor_record_offset":5044,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":255,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":5044,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":255,"record_offset":5788,"parent_record_offset":5760,"ancestor_record_offset":5044,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":255,"record_offset":5760,"parent_record_offset":5732,"ancestor_record_offset":5044,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":255,"record_offset":5944,"parent_record_offset":5760,"ancestor_record_offset":5044,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":255,"record_offset":6716,"parent_record_offset":6688,"ancestor_record_offset":5044,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":255,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":5044,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":255,"record_offset":5668,"parent_record_offset":5640,"ancestor_record_offset":5044,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":255,"record_offset":5612,"parent_record_offset":5584,"ancestor_record_offset":5044,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":255,"record_offset":5640,"parent_record_offset":5612,"ancestor_record_offset":5044,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":255,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":5044,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":255,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":5044,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":255,"record_offset":5528,"parent_record_offset":5500,"ancestor_record_offset":5044,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":255,"record_offset":6392,"parent_record_offset":6356,"ancestor_record_offset":5044,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":255,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":5044,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":255,"record_offset":5732,"parent_record_offset":5696,"ancestor_record_offset":5044,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":255,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":5044,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":255,"record_offset":5500,"parent_record_offset":5464,"ancestor_record_offset":5044,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":255,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":5044,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":255,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":5044,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":5044,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":255,"record_offset":5984,"parent_record_offset":5696,"ancestor_record_offset":5044,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":255,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":5044,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":255,"record_offset":6424,"parent_record_offset":6392,"ancestor_record_offset":5044,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":255,"record_offset":5368,"parent_record_offset":5332,"ancestor_record_offset":5044,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":255,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":5044,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":255,"record_offset":5396,"parent_record_offset":5332,"ancestor_record_offset":5044,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":255,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":5044,"inlinee":13747,"source_line":296,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":255,"record_offset":5332,"parent_record_offset":5204,"ancestor_record_offset":5044,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":255,"record_offset":6452,"parent_record_offset":6392,"ancestor_record_offset":5044,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":255,"record_offset":6484,"parent_record_offset":6356,"ancestor_record_offset":5044,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text_style
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":255,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":5044,"inlinee":6635,"source_line":153,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":255,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5044,"inlinee":5806,"source_line":32,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":255,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":5044,"inlinee":5422,"source_line":27,"archive":"_pdb/A/modules/00255.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152E
// 首条位置: {"module":12,"inlinee":5422}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":255,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":5264,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00255.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log
// 信息 ID: 84e11c55a786cc908bef4f10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 11c77c23cea2fdeb0f545043: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x18d72 -> 0x3c3bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xcaeed -> 0x3c3bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 信息 ID: 0259968382df99539667dce7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_error_log::do_gui_error_log; module=255, inline_record=10940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 信息 ID: 5d30a5fdd67ceed241dd86fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=255, inline_record=5044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 信息 ID: 8fd337550977f0e09baef566
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=255, inline_record=5264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
