// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/photomode/setup_photomode_toolbar.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 12a0fa5aff5f73dcb706a25635db6cdcf1f1441f996965311eb2f84c8c1dc2f8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: setup_photomode_toolbar.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4856, 4857
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
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4857,"record_offset":264,"rva":45294928,"va":5414004048}
// 编译实例 RVAs: 0x2b32550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=28
//     evidence={"assembly":"\tcallq\t-38325457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e82f33b7fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5414004140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-38331570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e84e1bb7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5414010269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=28
//     target_aliases=1699 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45281938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e86e0d4dfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5414004125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117fd20
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26951890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e82ebf64fe","size":5,"soft_fail":false,"target_va":5387058464,"va":5414010349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411820f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26941693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e803e764fe","size":5,"soft_fail":false,"target_va":5387067632,"va":5414009320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a990
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26902902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e88a7e65fe","size":5,"soft_fail":false,"target_va":5387102608,"va":5414005505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b31c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e8a9e2ffff","size":5,"soft_fail":false,"target_va":5414001696,"va":5414009202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=31
//     evidence={"assembly":"\tcallq\t2611322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e87ad82700","size":5,"soft_fail":false,"target_va":5416620769,"va":5414009442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32755351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e869310cfe","size":5,"soft_fail":false,"target_va":5381253904,"va":5414009250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e837b8ffff","size":5,"soft_fail":false,"target_va":5413990912,"va":5414009284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::link_up_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e847b4ffff","size":5,"soft_fail":false,"target_va":5413989968,"va":5414009348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5414004048,"raw_hex":"e8a3d2ffff","size":5,"soft_fail":false,"target_va":5413992512,"va":5414004120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4857,"record_offset":436,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4857,"record_offset":6672,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4857,"record_offset":6728,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4857,"record_offset":2584,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::button
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4857,"record_offset":984,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":145654,"source_line":85,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4857,"record_offset":2088,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":136638,"source_line":127,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":496,"parent_record_offset":464,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4857,"record_offset":7100,"parent_record_offset":7008,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4857,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4857,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":264,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4857,"record_offset":7284,"parent_record_offset":7260,"ancestor_record_offset":264,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4857,"record_offset":7036,"parent_record_offset":7008,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4857,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4857,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4857,"record_offset":7068,"parent_record_offset":7036,"ancestor_record_offset":264,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4857,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4857,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4857,"record_offset":6816,"parent_record_offset":6728,"ancestor_record_offset":264,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4857,"record_offset":6844,"parent_record_offset":6728,"ancestor_record_offset":264,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4857,"record_offset":6780,"parent_record_offset":6728,"ancestor_record_offset":264,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4857,"record_offset":6872,"parent_record_offset":6844,"ancestor_record_offset":264,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04857.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238F5
// 首条位置: {"module":4856,"inlinee":145653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":5952,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5824,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5824,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":6160,"parent_record_offset":6128,"ancestor_record_offset":5824,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":6128,"parent_record_offset":6100,"ancestor_record_offset":5824,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4856,"record_offset":6072,"parent_record_offset":6036,"ancestor_record_offset":5824,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":6008,"parent_record_offset":5980,"ancestor_record_offset":5824,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":6036,"parent_record_offset":6008,"ancestor_record_offset":5824,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":5824,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04856.jsonl.gz"}
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
// 完整结构及原始类型记录: setup_photomode_toolbar.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: setup_photomode_toolbar.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: setup_photomode_toolbar.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 信息 ID: 796e7cf7d3460dcd6f14c1b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::toolbar_setup @ 0x296a0e1 -> 0x2b32550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 信息 ID: 94612fc8c762e592c85af3f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; module=4856, inline_record=5824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
