// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/gui_error_log.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 517f1d9e7f2ba2a6e13135cd8130a43908ef622535379048dddbc6587dbe63bd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_error_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 204, 238
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
// 首条位置: {"module":238,"record_offset":248,"rva":6303792,"va":5375012912}
// 编译实例 RVAs: 0x603030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t665614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e80e280a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5375013069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6290867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e84d02a0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375013054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e910
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5655553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e8ffb3a9ff","size":5,"soft_fail":false,"target_va":5369358608,"va":5375014156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054cb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-747216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e83099f4ff","size":5,"soft_fail":false,"target_va":5374266160,"va":5375013371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e87b160000","size":5,"soft_fail":false,"target_va":5375019776,"va":5375014016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t815134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e81e700c00","size":5,"soft_fail":false,"target_va":5375829056,"va":5375013917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t936719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e80f4b0e00","size":5,"soft_fail":false,"target_va":5375950464,"va":5375013740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gui_error_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42043208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e848878102","size":5,"soft_fail":false,"target_va":5417057056,"va":5375013843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41606685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e81dde7a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375014079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41606692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e824de7a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375014097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7757848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e818607600","size":5,"soft_fail":false,"target_va":5382771904,"va":5375014051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::widget_text::WidgetText::fallback_text_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375012912,"raw_hex":"e8c3480000","size":5,"soft_fail":false,"target_va":5375031856,"va":5375013224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":238,"record_offset":404,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8436,"source_line":135,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":2596,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::anchor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":238,"record_offset":2328,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13243,"source_line":239,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::collapsible
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":238,"record_offset":2264,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17449,"source_line":307,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::frame
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":238,"record_offset":2292,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17450,"source_line":125,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":238,"record_offset":696,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17448,"source_line":49,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":238,"record_offset":2360,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7250,"source_line":365,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] ahash::random_state::RandomState::with_seeds
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":238,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":248,"inlinee":7246,"source_line":308,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":238,"record_offset":2432,"parent_record_offset":2392,"ancestor_record_offset":248,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":238,"record_offset":2392,"parent_record_offset":2360,"ancestor_record_offset":248,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":952,"parent_record_offset":924,"ancestor_record_offset":248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":248,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":248,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":238,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":248,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":238,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":248,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":238,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":248,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":238,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":248,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":238,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":248,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":238,"record_offset":1836,"parent_record_offset":1384,"ancestor_record_offset":248,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":238,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":248,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":248,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":238,"record_offset":740,"parent_record_offset":696,"ancestor_record_offset":248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::count::to_usize::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":238,"record_offset":544,"parent_record_offset":524,"ancestor_record_offset":248,"inlinee":8438,"source_line":138,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":238,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":248,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":238,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":238,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":238,"record_offset":644,"parent_record_offset":524,"ancestor_record_offset":248,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":238,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":248,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":248,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":238,"record_offset":1288,"parent_record_offset":1008,"ancestor_record_offset":248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":238,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":248,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":238,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":2820,"parent_record_offset":2760,"ancestor_record_offset":248,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":2852,"parent_record_offset":2724,"ancestor_record_offset":248,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::id::Id::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/id.rs
//     evidence={"module":238,"record_offset":2192,"parent_record_offset":696,"ancestor_record_offset":248,"inlinee":7245,"source_line":54,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":804,"parent_record_offset":772,"ancestor_record_offset":248,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":1612,"parent_record_offset":1384,"ancestor_record_offset":248,"inlinee":6121,"source_line":92,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::WidgetText::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":1384,"parent_record_offset":696,"ancestor_record_offset":248,"inlinee":6116,"source_line":477,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$8::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":248,"inlinee":8130,"source_line":693,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":238,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":248,"inlinee":17447,"source_line":12,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4427
// 首条位置: {"module":238,"inlinee":17447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":572,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4431
// 首条位置: {"module":238,"inlinee":17457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":238,"record_offset":6792,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":238,"record_offset":6848,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":7764,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":7848,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":7972,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":238,"record_offset":7916,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":7884,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":7358,"source_line":254,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::heading
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":7792,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":7355,"source_line":167,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":7072,"parent_record_offset":6712,"ancestor_record_offset":6712,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":6712,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":6712,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":6712,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":6712,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":6712,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":8552,"parent_record_offset":8528,"ancestor_record_offset":6712,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":7548,"parent_record_offset":7400,"ancestor_record_offset":6712,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":8480,"parent_record_offset":8424,"ancestor_record_offset":6712,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":7340,"parent_record_offset":7312,"ancestor_record_offset":6712,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":6712,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":6712,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":7400,"parent_record_offset":7368,"ancestor_record_offset":6712,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":7368,"parent_record_offset":7340,"ancestor_record_offset":6712,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":6712,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":238,"record_offset":7436,"parent_record_offset":7400,"ancestor_record_offset":6712,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":7220,"parent_record_offset":7192,"ancestor_record_offset":6712,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":7248,"parent_record_offset":7220,"ancestor_record_offset":6712,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":6712,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":7192,"parent_record_offset":7164,"ancestor_record_offset":6712,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":238,"record_offset":7164,"parent_record_offset":7136,"ancestor_record_offset":6712,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":238,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":6712,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":238,"record_offset":8080,"parent_record_offset":8044,"ancestor_record_offset":6712,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":238,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":6712,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":7312,"parent_record_offset":7276,"ancestor_record_offset":6712,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":238,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":6712,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":238,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":6712,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":238,"record_offset":7108,"parent_record_offset":7072,"ancestor_record_offset":6712,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":238,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":6712,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":238,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":6712,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":238,"record_offset":6980,"parent_record_offset":6876,"ancestor_record_offset":6712,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":6712,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":238,"record_offset":7676,"parent_record_offset":7276,"ancestor_record_offset":6712,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":6712,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":6712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":238,"record_offset":8112,"parent_record_offset":8080,"ancestor_record_offset":6712,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":238,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":6712,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":238,"record_offset":6876,"parent_record_offset":6848,"ancestor_record_offset":6712,"inlinee":14600,"source_line":343,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":238,"record_offset":6912,"parent_record_offset":6876,"ancestor_record_offset":6712,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":8140,"parent_record_offset":8080,"ancestor_record_offset":6712,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":8172,"parent_record_offset":8044,"ancestor_record_offset":6712,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::text_style
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":238,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":6712,"inlinee":7356,"source_line":153,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":238,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":6712,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_error_log.rs
//     evidence={"module":238,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6712,"inlinee":16234,"source_line":27,"archive":"_pdb/C/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F6A
// 首条位置: {"module":204,"inlinee":16234}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":238,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":7004,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00238.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log
// 信息 ID: 3e35c2b6cc1431c1cf2a3767
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 11c77c23cea2fdeb0f545043: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x2d0d81 -> 0x603030 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x578373 -> 0x603030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$0
// 信息 ID: 5795bfcd166eee7b33a03b8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_error_log::do_gui_error_log; module=238, inline_record=572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1
// 信息 ID: cce0e03565ea9f2f554bbcb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=238, inline_record=6712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_error_log::do_gui_error_log::closure$1::closure$0
// 信息 ID: 1d39fe328b9c06dc097b8126
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_error_log::do_gui_error_log", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_error_log::do_gui_error_log::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > >; module=238, inline_record=7004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
