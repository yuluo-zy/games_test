// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/toolbar_startup.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3e666e4a5b5019de3ac3b2b9b6ac4faf252d862eae6f2a47e0b0c477551ba323
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_startup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4202, 4856
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
// 函数符号: toolbar::toolbar_startup::setup_toolbar
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4202,"record_offset":264,"rva":43414816,"va":5412123936}
// 编译实例 RVAs: 0x2967520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=32
//     evidence={"assembly":"\tcallq\t-36445345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e85fe3d3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412124028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-36453330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e82ec4d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412132029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=32
//     target_aliases=1699 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43401826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e89ebd69fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412124013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117fd20
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25074002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e8ae6681fe","size":5,"soft_fail":false,"target_va":5387058464,"va":5412132461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411820f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25063264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e8a09081fe","size":5,"soft_fail":false,"target_va":5387067632,"va":5412130891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a990
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25029602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e81e1482fe","size":5,"soft_fail":false,"target_va":5387102608,"va":5412132205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b315e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1869318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e806861c00","size":5,"soft_fail":false,"target_va":5414000096,"va":5412130773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=34
//     evidence={"assembly":"\tcallq\t4495415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e837984400","size":5,"soft_fail":false,"target_va":5416620769,"va":5412125349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30876922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e806db28fe","size":5,"soft_fail":false,"target_va":5381253904,"va":5412130821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1860052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e8d4611c00","size":5,"soft_fail":false,"target_va":5413990912,"va":5412130855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::link_up_nav
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1859044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e8e45d1c00","size":5,"soft_fail":false,"target_va":5413989968,"va":5412130919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1868499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412123936,"raw_hex":"e8d3821c00","size":5,"soft_fail":false,"target_va":5413992512,"va":5412124008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4202,"record_offset":408,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4202,"record_offset":6892,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4202,"record_offset":6948,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4202,"record_offset":7132,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::column
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4202,"record_offset":6128,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":136639,"source_line":137,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4202,"record_offset":768,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":136638,"source_line":127,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":468,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":7320,"parent_record_offset":7228,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4202,"record_offset":436,"parent_record_offset":408,"ancestor_record_offset":264,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4202,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":264,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":7256,"parent_record_offset":7228,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":7204,"parent_record_offset":7180,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4202,"record_offset":7288,"parent_record_offset":7256,"ancestor_record_offset":264,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4202,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4202,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4202,"record_offset":7036,"parent_record_offset":6948,"ancestor_record_offset":264,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4202,"record_offset":7064,"parent_record_offset":6948,"ancestor_record_offset":264,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4202,"record_offset":7000,"parent_record_offset":6948,"ancestor_record_offset":264,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4202,"record_offset":7092,"parent_record_offset":7064,"ancestor_record_offset":264,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::toolbar_setup
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4202,"record_offset":9724,"rva":43425792,"va":5412134912}
// 编译实例 RVAs: 0x296a000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-36456360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e858b8d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412135059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141183190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25063128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e8289181fe","size":5,"soft_fail":false,"target_va":5387071888,"va":5412135011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411cb9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24766054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e89a1986fe","size":5,"soft_fail":false,"target_va":5387368912,"va":5412134961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142967520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e856d4ffff","size":5,"soft_fail":false,"target_va":5412123936,"va":5412135109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4217652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e8345b4000","size":5,"soft_fail":false,"target_va":5416353040,"va":5412135383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4921974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e8761a4b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5412135077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24821599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e8a14085fe","size":5,"soft_fail":false,"target_va":5387313616,"va":5412135210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1868906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e86a841c00","size":5,"soft_fail":false,"target_va":5414004048,"va":5412135137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1877830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412134912,"raw_hex":"e846a71c00","size":5,"soft_fail":false,"target_va":5414013008,"va":5412135173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4202,"record_offset":9888,"parent_record_offset":9724,"ancestor_record_offset":9724,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4202,"record_offset":10716,"parent_record_offset":9724,"ancestor_record_offset":9724,"inlinee":11204,"source_line":840,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4202,"record_offset":9944,"parent_record_offset":9724,"ancestor_record_offset":9724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":4202,"record_offset":9860,"parent_record_offset":9724,"ancestor_record_offset":9724,"inlinee":22073,"source_line":49,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9724,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":10172,"parent_record_offset":10148,"ancestor_record_offset":9724,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":9724,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4202,"record_offset":10124,"parent_record_offset":10036,"ancestor_record_offset":9724,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":9724,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10916,"parent_record_offset":10892,"ancestor_record_offset":9724,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10064,"parent_record_offset":10036,"ancestor_record_offset":9724,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9724,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10892,"parent_record_offset":10860,"ancestor_record_offset":9724,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":9724,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4202,"record_offset":10012,"parent_record_offset":9992,"ancestor_record_offset":9724,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4202,"record_offset":10804,"parent_record_offset":10772,"ancestor_record_offset":9724,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4202,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":9724,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4202,"record_offset":10744,"parent_record_offset":10716,"ancestor_record_offset":9724,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4202,"record_offset":10772,"parent_record_offset":10744,"ancestor_record_offset":9724,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4202,"record_offset":10092,"parent_record_offset":10064,"ancestor_record_offset":9724,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4202,"record_offset":9972,"parent_record_offset":9944,"ancestor_record_offset":9724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::setup_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 351
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238F1
// 首条位置: {"module":4856,"inlinee":145649}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":2768,"parent_record_offset":2704,"ancestor_record_offset":2704,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":2920,"parent_record_offset":2892,"ancestor_record_offset":2704,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2704,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":2980,"parent_record_offset":2948,"ancestor_record_offset":2704,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4856,"record_offset":2948,"parent_record_offset":2920,"ancestor_record_offset":2704,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4856,"record_offset":2892,"parent_record_offset":2856,"ancestor_record_offset":2704,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":2824,"parent_record_offset":2796,"ancestor_record_offset":2704,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":2704,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04856.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4856,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2704,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04856.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_startup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_startup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_startup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar
// 信息 ID: 0cc4501c6469a63de762b6ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::toolbar_setup @ 0x296a0c5 -> 0x2967520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar::closure$0
// 信息 ID: e315de8053d48dd5958b5804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_startup::setup_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_startup::setup_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; module=4856, inline_record=2704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::toolbar_setup
// 信息 ID: f9aa36a9b6236fa60ca6e84e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c1d24fb5ae7f95d1a0f0763: void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)> > @ 0x2309eef -> 0x296a000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::commands::Commands)> @ 0x2958d2c -> 0x296a000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
