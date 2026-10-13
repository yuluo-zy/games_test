// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/load.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 24366bc57e3f73bdc439153648a2d4f4f4005438180486c71226754444367ad7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1235, 1239, 1296, 2873, 2875, 2893, 2948, 4247, 4865
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":209656,"rva":18474336,"va":5387183456}
// 编译实例 RVAs: 0x119e560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2529628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8a466d9ff","size":5,"soft_fail":false,"target_va":5384656400,"va":5387186023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11508363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8756550ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387187062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17847248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e830aceffe","size":5,"soft_fail":false,"target_va":5369338848,"va":5387186091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=13
//     target_aliases=6 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13704492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8d4e22eff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387183527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140731780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10934293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8eb2759ff","size":5,"soft_fail":false,"target_va":5376251776,"va":5387186064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177200
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-160939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8558bfdff","size":5,"soft_fail":false,"target_va":5387022848,"va":5387183782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cec0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-137187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e81de8fdff","size":5,"soft_fail":false,"target_va":5387046592,"va":5387183774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29164732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8bc04bd01","size":5,"soft_fail":false,"target_va":5416350720,"va":5387185983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29434161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e83121c101","size":5,"soft_fail":false,"target_va":5416621536,"va":5387187370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29437582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e88e2ec101","size":5,"soft_fail":false,"target_va":5416621600,"va":5387184013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29434273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8a121c101","size":5,"soft_fail":false,"target_va":5416621762,"va":5387187484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6918633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387183456,"raw_hex":"e8176e96ff","size":5,"soft_fail":false,"target_va":5380267312,"va":5387185940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1239,"record_offset":219996,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":220344,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":219912,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":209824,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":21726,"source_line":388,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":210908,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":21802,"source_line":1339,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::size
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":214660,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":21710,"source_line":1344,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":210968,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":70197,"source_line":254,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$54::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":217792,"parent_record_offset":209656,"ancestor_record_offset":209656,"inlinee":21691,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":224252,"parent_record_offset":224228,"ancestor_record_offset":209656,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":224276,"parent_record_offset":224252,"ancestor_record_offset":209656,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":224304,"parent_record_offset":224276,"ancestor_record_offset":209656,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":224228,"parent_record_offset":224172,"ancestor_record_offset":209656,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":220208,"parent_record_offset":220184,"ancestor_record_offset":209656,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":224200,"parent_record_offset":224172,"ancestor_record_offset":209656,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":224172,"parent_record_offset":224148,"ancestor_record_offset":209656,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":220256,"parent_record_offset":220232,"ancestor_record_offset":209656,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":220232,"parent_record_offset":220208,"ancestor_record_offset":209656,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":224148,"parent_record_offset":224124,"ancestor_record_offset":209656,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":220184,"parent_record_offset":220148,"ancestor_record_offset":209656,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":220120,"parent_record_offset":220084,"ancestor_record_offset":209656,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":220148,"parent_record_offset":220120,"ancestor_record_offset":209656,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":220056,"parent_record_offset":220020,"ancestor_record_offset":209656,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":220020,"parent_record_offset":219996,"ancestor_record_offset":209656,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1239,"record_offset":220084,"parent_record_offset":220020,"ancestor_record_offset":209656,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":210580,"parent_record_offset":210556,"ancestor_record_offset":209656,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":210148,"parent_record_offset":210124,"ancestor_record_offset":209656,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":210232,"parent_record_offset":210208,"ancestor_record_offset":209656,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":210648,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":210380,"parent_record_offset":210352,"ancestor_record_offset":209656,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":210352,"parent_record_offset":210324,"ancestor_record_offset":209656,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":223964,"parent_record_offset":223940,"ancestor_record_offset":209656,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":210876,"parent_record_offset":209824,"ancestor_record_offset":209656,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":210172,"parent_record_offset":210148,"ancestor_record_offset":209656,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":224028,"parent_record_offset":224004,"ancestor_record_offset":209656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":210496,"parent_record_offset":210472,"ancestor_record_offset":209656,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":211040,"parent_record_offset":211008,"ancestor_record_offset":209656,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":209852,"parent_record_offset":209824,"ancestor_record_offset":209656,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":220296,"parent_record_offset":220148,"ancestor_record_offset":209656,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":210604,"parent_record_offset":210580,"ancestor_record_offset":209656,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":210292,"parent_record_offset":210264,"ancestor_record_offset":209656,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":210324,"parent_record_offset":210292,"ancestor_record_offset":209656,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":210264,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":210124,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":210676,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":210208,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":210068,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":210556,"parent_record_offset":210532,"ancestor_record_offset":209656,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":209908,"parent_record_offset":209880,"ancestor_record_offset":209656,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":211096,"parent_record_offset":211068,"ancestor_record_offset":209656,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":210532,"parent_record_offset":210424,"ancestor_record_offset":209656,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":209940,"parent_record_offset":209908,"ancestor_record_offset":209656,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210472,"parent_record_offset":210448,"ancestor_record_offset":209656,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210768,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210448,"parent_record_offset":210424,"ancestor_record_offset":209656,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":209992,"parent_record_offset":209968,"ancestor_record_offset":209656,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":209968,"parent_record_offset":209908,"ancestor_record_offset":209656,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":211156,"parent_record_offset":211096,"ancestor_record_offset":209656,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210016,"parent_record_offset":209992,"ancestor_record_offset":209656,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210096,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":210424,"parent_record_offset":210016,"ancestor_record_offset":209656,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":209880,"parent_record_offset":209824,"ancestor_record_offset":209656,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":211068,"parent_record_offset":211008,"ancestor_record_offset":209656,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":217868,"parent_record_offset":217816,"ancestor_record_offset":209656,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":210820,"parent_record_offset":209824,"ancestor_record_offset":209656,"inlinee":21727,"source_line":350,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":212044,"parent_record_offset":212016,"ancestor_record_offset":209656,"inlinee":22049,"source_line":330,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":212100,"parent_record_offset":212016,"ancestor_record_offset":209656,"inlinee":22041,"source_line":325,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":214692,"parent_record_offset":214660,"ancestor_record_offset":209656,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":217816,"parent_record_offset":217792,"ancestor_record_offset":209656,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":211008,"parent_record_offset":210968,"ancestor_record_offset":209656,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":212068,"parent_record_offset":212044,"ancestor_record_offset":209656,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":210844,"parent_record_offset":210820,"ancestor_record_offset":209656,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":219936,"parent_record_offset":219912,"ancestor_record_offset":209656,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":219960,"parent_record_offset":219936,"ancestor_record_offset":209656,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":210936,"parent_record_offset":210908,"ancestor_record_offset":209656,"inlinee":21722,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":212016,"parent_record_offset":210968,"ancestor_record_offset":209656,"inlinee":70216,"source_line":728,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":213272,"parent_record_offset":212224,"ancestor_record_offset":209656,"inlinee":70218,"source_line":753,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":214512,"parent_record_offset":213480,"ancestor_record_offset":209656,"inlinee":70217,"source_line":773,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":222612,"parent_record_offset":221576,"ancestor_record_offset":209656,"inlinee":70219,"source_line":792,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11556
// 首条位置: {"module":1296,"inlinee":70998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2893,"record_offset":7248,"rva":36988128,"va":5405697248}
// 编译实例 RVAs: 0x23464e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20909242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e846f3c0fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5405705221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21043248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8d0e7befe","size":5,"soft_fail":false,"target_va":5384656240,"va":5405699483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-20930767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8319fc0fe","size":5,"soft_fail":false,"target_va":5384768864,"va":5405699626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-21051581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e843c7befe","size":5,"soft_fail":false,"target_va":5384648032,"va":5405699608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30025531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8c5d835fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5405704214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=28
//     evidence={"assembly":"\tcallq\t-30020815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e831eb35fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405699514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36982014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e802b3cbfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5405704201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=7
//     target_aliases=4 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36260078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e812b7d6fd","size":5,"soft_fail":false,"target_va":5369439344,"va":5405699417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401550d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35593682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e82ee2e0fd","size":5,"soft_fail":false,"target_va":5370106064,"va":5405699741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=7
//     target_aliases=6 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32219416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8e85e14fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405698451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32217299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e82d6714fe","size":5,"soft_fail":false,"target_va":5373484384,"va":5405701678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32216926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8a26814fe","size":5,"soft_fail":false,"target_va":5373484784,"va":5405701705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29465307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e825653efe","size":5,"soft_fail":false,"target_va":5376236864,"va":5405702166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27573379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e87d435bfe","size":5,"soft_fail":false,"target_va":5378127136,"va":5405700510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27113395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e84d4862fe","size":5,"soft_fail":false,"target_va":5378598624,"va":5405712014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05d10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25431867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8c5f07bfe","size":5,"soft_fail":false,"target_va":5380267280,"va":5405699142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b08cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25424120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8080f7cfe","size":5,"soft_fail":false,"target_va":5380279536,"va":5405703651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24633710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8921e88fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405697881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24091575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8496490fe","size":5,"soft_fail":false,"target_va":5381607664,"va":5405699234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141173c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18689357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8b3d2e2fe","size":5,"soft_fail":false,"target_va":5387009056,"va":5405698408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141176510
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18680868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8dcf3e2fe","size":5,"soft_fail":false,"target_va":5387019536,"va":5405700399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141176860
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18678299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8e5fde2fe","size":5,"soft_fail":false,"target_va":5387020384,"va":5405698678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18681448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e898f1e2fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5405702179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b2b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18659530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e83647e3fe","size":5,"soft_fail":false,"target_va":5387039408,"va":5405698933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18654462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8025be3fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5405700441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141189950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18607860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e80c11e4fe","size":5,"soft_fail":false,"target_va":5387098448,"va":5405706303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18610989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8d304e4fe","size":5,"soft_fail":false,"target_va":5387100208,"va":5405711192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ee40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18581704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e83877e4fe","size":5,"soft_fail":false,"target_va":5387120192,"va":5405701891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18584105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8d76de4fe","size":5,"soft_fail":false,"target_va":5387124336,"va":5405708436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141191c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18568811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e895a9e4fe","size":5,"soft_fail":false,"target_va":5387131904,"va":5405700710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411929c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18565048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e848b8e4fe","size":5,"soft_fail":false,"target_va":5387135424,"va":5405700467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141195c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18559690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e836cde4fe","size":5,"soft_fail":false,"target_va":5387148320,"va":5405708005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411981b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18550013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e803f3e4fe","size":5,"soft_fail":false,"target_va":5387157936,"va":5405707944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14119a080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18541611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8d513e5fe","size":5,"soft_fail":false,"target_va":5387165824,"va":5405707430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c19e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18379681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e85f8ce7fe","size":5,"soft_fail":false,"target_va":5387327968,"va":5405707644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c5b90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18363909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8fbc9e7fe","size":5,"soft_fail":false,"target_va":5387344784,"va":5405708688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419aff00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10055013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e89b9266ff","size":5,"soft_fail":false,"target_va":5395644160,"va":5405699168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10615482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8bafaa100","size":5,"soft_fail":false,"target_va":5416312752,"va":5405697265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10648102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8267aa200","size":5,"soft_fail":false,"target_va":5416350944,"va":5405702837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10644580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8646ca200","size":5,"soft_fail":false,"target_va":5416351056,"va":5405706471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10653429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8f58ea200","size":5,"soft_fail":false,"target_va":5416351392,"va":5405697958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11353311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8df3cad00","size":5,"soft_fail":false,"target_va":5417057056,"va":5405703740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8a5ecffff","size":5,"soft_fail":false,"target_va":5405694736,"va":5405699686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20909309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e803f3c0fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5405704168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10914221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8ad89a600","size":5,"soft_fail":false,"target_va":5416620794,"va":5405706568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21075772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8c468befe","size":5,"soft_fail":false,"target_va":5384624080,"va":5405699847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18387160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8286fe7fe","size":5,"soft_fail":false,"target_va":5387312896,"va":5405700051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10912856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e85884a600","size":5,"soft_fail":false,"target_va":5416621408,"va":5405708547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10917840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8d097a600","size":5,"soft_fail":false,"target_va":5416621536,"va":5405703691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10922409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8a9a9a600","size":5,"soft_fail":false,"target_va":5416621600,"va":5405699186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::session::SessionMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18595396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8bc41e4fe","size":5,"soft_fail":false,"target_va":5387114704,"va":5405710095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::session::NextSessionKind> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-18585148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8c469e4fe","size":5,"soft_fail":false,"target_va":5387114928,"va":5405700071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::unwrap_or_default<country_core::resources::session::SessionMeta,anyhow::Error>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8b7f3ffff","size":5,"soft_fail":false,"target_va":5405696224,"va":5405699364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10913685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e89587a600","size":5,"soft_fail":false,"target_va":5416622320,"va":5405708630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e85cc7ffff","size":5,"soft_fail":false,"target_va":5405694336,"va":5405708831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e868f7ffff","size":5,"soft_fail":false,"target_va":5405695168,"va":5405697363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10912642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e88283a600","size":5,"soft_fail":false,"target_va":5416621376,"va":5405708729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25438225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8efd77bfe","size":5,"soft_fail":false,"target_va":5380261184,"va":5405699404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25439692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e834d27bfe","size":5,"soft_fail":false,"target_va":5380267776,"va":5405707463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25432126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8c2ef7bfe","size":5,"soft_fail":false,"target_va":5380267312,"va":5405699433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::assets_root
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25437277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8a3db7bfe","size":5,"soft_fail":false,"target_va":5380262112,"va":5405699384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::write_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25443778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e83ec27bfe","size":5,"soft_fail":false,"target_va":5380262240,"va":5405706013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25250680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e888b47efe","size":5,"soft_fail":false,"target_va":5380448448,"va":5405699123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_game::parse_saved_game_data
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25318222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8b2ac7dfe","size":5,"soft_fail":false,"target_va":5380381344,"va":5405699561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24721614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e832c786fe","size":5,"soft_fail":false,"target_va":5380985888,"va":5405707497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22567425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8ffa5a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405702140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21733924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8dc5db4fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5405697711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21725417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8177fb4fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5405697636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21733396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8ec5fb4fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5405698303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21734527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8815bb4fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5405697338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e811e8ffff","size":5,"soft_fail":false,"target_va":5405694880,"va":5405701002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10947998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e89e0da700","size":5,"soft_fail":false,"target_va":5416656560,"va":5405708557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18377952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e82093e7fe","size":5,"soft_fail":false,"target_va":5387331072,"va":5405709019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10910756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8247ca600","size":5,"soft_fail":false,"target_va":5416619360,"va":5405708599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::canonical_mod_entries
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22787279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8314ba4fe","size":5,"soft_fail":false,"target_va":5382918496,"va":5405705770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29491550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8a2fe3dfe","size":5,"soft_fail":false,"target_va":5376206368,"va":5405697913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29493365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e88bf73dfe","size":5,"soft_fail":false,"target_va":5376208368,"va":5405701728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::hover
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29491870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e862fd3dfe","size":5,"soft_fail":false,"target_va":5376209296,"va":5405701161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::import::entry_ui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6620607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8bf056500","size":5,"soft_fail":false,"target_va":5412328784,"va":5405708172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::import::ui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6624052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e834136500","size":5,"soft_fail":false,"target_va":5412322304,"va":5405698247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::display_save_file_load_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e8bbefffff","size":5,"soft_fail":false,"target_va":5405697136,"va":5405701296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::load_session_meta
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e83bf4ffff","size":5,"soft_fail":false,"target_va":5405696336,"va":5405699344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::show_contextual_buttons
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e87defffff","size":5,"soft_fail":false,"target_va":5405696624,"va":5405700846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::focus_item
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-480430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697248,"raw_hex":"e852abf8ff","size":5,"soft_fail":false,"target_va":5405218336,"va":5405698761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2893,"record_offset":26348,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":32652,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2893,"record_offset":11816,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":27288,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":32880,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2893,"record_offset":9764,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2893,"record_offset":20844,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_state","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.17.3/src/state/resources.rs
//     evidence={"module":2893,"record_offset":14284,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":10613,"source_line":134,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":2893,"record_offset":23012,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":15944,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":32824,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":13604,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":15744,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2893,"record_offset":14140,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2893,"record_offset":32200,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":10292,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":15716,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":13432,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":9824,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":10232,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":27196,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":10740,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2893,"record_offset":22912,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":5420,"source_line":1265,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2893,"record_offset":32052,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$35::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":14312,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":70225,"source_line":325,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2893,"record_offset":14936,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":8956,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":7556,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":7944,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2893,"record_offset":13744,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2893,"record_offset":14992,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":9912,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":10267,"source_line":466,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":16660,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":33936,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":10270,"source_line":483,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":19520,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":16980,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::default_focus
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":17232,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":22025,"source_line":1537,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":18304,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":21795,"source_line":398,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":18364,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":21798,"source_line":357,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":18424,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::import::LocalState::reset_availability
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/import.rs
//     evidence={"module":2893,"record_offset":10592,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111802,"source_line":43,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ui::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2893,"record_offset":23312,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111803,"source_line":317,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::abandon_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13320,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111438,"source_line":116,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::clicked_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13460,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111436,"source_line":112,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::hovered_item
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13376,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111437,"source_line":104,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::hovered_item_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":14768,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111440,"source_line":108,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::initial_focused_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13072,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111442,"source_line":120,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::lock
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13260,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111444,"source_line":173,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::min_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13136,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111439,"source_line":179,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::nav_scroll_speed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/thumbnail_scroll.rs
//     evidence={"module":2893,"record_offset":13196,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111441,"source_line":185,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":11904,"parent_record_offset":7248,"ancestor_record_offset":7248,"inlinee":111554,"source_line":254,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":7248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":7248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":7248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":14044,"parent_record_offset":14020,"ancestor_record_offset":7248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":24004,"parent_record_offset":23980,"ancestor_record_offset":7248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":7248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23932,"parent_record_offset":23784,"ancestor_record_offset":7248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":13996,"parent_record_offset":13940,"ancestor_record_offset":7248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2893,"record_offset":28424,"parent_record_offset":28396,"ancestor_record_offset":7248,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2893,"record_offset":23392,"parent_record_offset":23312,"ancestor_record_offset":7248,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2893,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":7248,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":10100,"parent_record_offset":10068,"ancestor_record_offset":7248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":7248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":7248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":13940,"parent_record_offset":13916,"ancestor_record_offset":7248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":7248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":7248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23784,"parent_record_offset":23752,"ancestor_record_offset":7248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23752,"parent_record_offset":23724,"ancestor_record_offset":7248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":7248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23820,"parent_record_offset":23784,"ancestor_record_offset":7248,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23604,"parent_record_offset":23560,"ancestor_record_offset":7248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23632,"parent_record_offset":23604,"ancestor_record_offset":7248,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":27396,"parent_record_offset":27324,"ancestor_record_offset":7248,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23660,"parent_record_offset":23632,"ancestor_record_offset":7248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23560,"parent_record_offset":23532,"ancestor_record_offset":7248,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":32684,"parent_record_offset":32652,"ancestor_record_offset":7248,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2893,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":7248,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2893,"record_offset":23504,"parent_record_offset":23476,"ancestor_record_offset":7248,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":10068,"parent_record_offset":10032,"ancestor_record_offset":7248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":27368,"parent_record_offset":27324,"ancestor_record_offset":7248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":10008,"parent_record_offset":9936,"ancestor_record_offset":7248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":7248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":23696,"parent_record_offset":23660,"ancestor_record_offset":7248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":32224,"parent_record_offset":32200,"ancestor_record_offset":7248,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":7248,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":27324,"parent_record_offset":27288,"ancestor_record_offset":7248,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2893,"record_offset":9792,"parent_record_offset":9764,"ancestor_record_offset":7248,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2893,"record_offset":20868,"parent_record_offset":20844,"ancestor_record_offset":7248,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2893,"record_offset":20932,"parent_record_offset":20868,"ancestor_record_offset":7248,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2893,"record_offset":23844,"parent_record_offset":23820,"ancestor_record_offset":7248,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":7248,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":7248,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2893,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":7248,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2893,"record_offset":23152,"parent_record_offset":23060,"ancestor_record_offset":7248,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":7248,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9312,"parent_record_offset":9284,"ancestor_record_offset":7248,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9232,"parent_record_offset":9204,"ancestor_record_offset":7248,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":7248,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9372,"parent_record_offset":9256,"ancestor_record_offset":7248,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2893,"record_offset":9484,"parent_record_offset":9460,"ancestor_record_offset":7248,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":15972,"parent_record_offset":15944,"ancestor_record_offset":7248,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2893,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":7248,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2893,"record_offset":23476,"parent_record_offset":23312,"ancestor_record_offset":7248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2893,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":7248,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2893,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":7248,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2893,"record_offset":12776,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2893,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":7248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2893,"record_offset":27888,"parent_record_offset":27848,"ancestor_record_offset":7248,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":2893,"record_offset":27848,"parent_record_offset":27396,"ancestor_record_offset":7248,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2893,"record_offset":32248,"parent_record_offset":32224,"ancestor_record_offset":7248,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2893,"record_offset":27980,"parent_record_offset":27396,"ancestor_record_offset":7248,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2893,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":7248,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2893,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":7248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2893,"record_offset":32972,"parent_record_offset":32936,"ancestor_record_offset":7248,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2893,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":7248,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2893,"record_offset":33664,"parent_record_offset":33636,"ancestor_record_offset":7248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2893,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":7248,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2893,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":7248,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2893,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":7248,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2893,"record_offset":9104,"parent_record_offset":9064,"ancestor_record_offset":7248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":2893,"record_offset":23096,"parent_record_offset":23060,"ancestor_record_offset":7248,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":2893,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":7248,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":7248,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":23416,"parent_record_offset":23392,"ancestor_record_offset":7248,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":12948,"parent_record_offset":11940,"ancestor_record_offset":7248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":7248,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2893,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7248,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2893,"record_offset":9720,"parent_record_offset":9064,"ancestor_record_offset":7248,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2893,"record_offset":24060,"parent_record_offset":23660,"ancestor_record_offset":7248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2893,"record_offset":23236,"parent_record_offset":23176,"ancestor_record_offset":7248,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":7248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=151
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":9436,"parent_record_offset":9204,"ancestor_record_offset":7248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2893,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":7248,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":11976,"parent_record_offset":11940,"ancestor_record_offset":7248,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":18488,"parent_record_offset":18456,"ancestor_record_offset":7248,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7248,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":7248,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":8548,"parent_record_offset":8520,"ancestor_record_offset":7248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":23260,"parent_record_offset":23236,"ancestor_record_offset":7248,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":10188,"parent_record_offset":10032,"ancestor_record_offset":7248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2893,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":7248,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2893,"record_offset":8648,"parent_record_offset":8620,"ancestor_record_offset":7248,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2893,"record_offset":8620,"parent_record_offset":8592,"ancestor_record_offset":7248,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2893,"record_offset":23176,"parent_record_offset":23152,"ancestor_record_offset":7248,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2893,"record_offset":33004,"parent_record_offset":32936,"ancestor_record_offset":7248,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2893,"record_offset":32908,"parent_record_offset":32880,"ancestor_record_offset":7248,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2893,"record_offset":23208,"parent_record_offset":23176,"ancestor_record_offset":7248,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2893,"record_offset":32936,"parent_record_offset":32908,"ancestor_record_offset":7248,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":7248,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":7248,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":8676,"parent_record_offset":8648,"ancestor_record_offset":7248,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":10376,"parent_record_offset":10344,"ancestor_record_offset":7248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::position
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":10344,"parent_record_offset":10320,"ancestor_record_offset":7248,"inlinee":6398,"source_line":377,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":8268,"parent_record_offset":8152,"ancestor_record_offset":7248,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":2893,"record_offset":32716,"parent_record_offset":32684,"ancestor_record_offset":7248,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2893,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":7248,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2893,"record_offset":7980,"parent_record_offset":7944,"ancestor_record_offset":7248,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2893,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":7248,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2893,"record_offset":8088,"parent_record_offset":8052,"ancestor_record_offset":7248,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2893,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":7248,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2893,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":7248,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2893,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":7248,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2893,"record_offset":8592,"parent_record_offset":8052,"ancestor_record_offset":7248,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":7248,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":7248,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7784,"parent_record_offset":7752,"ancestor_record_offset":7248,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7840,"parent_record_offset":7784,"ancestor_record_offset":7248,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":7248,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7600,"parent_record_offset":7556,"ancestor_record_offset":7248,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2893,"record_offset":7864,"parent_record_offset":7840,"ancestor_record_offset":7248,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2893,"record_offset":8152,"parent_record_offset":8120,"ancestor_record_offset":7248,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2893,"record_offset":8436,"parent_record_offset":8152,"ancestor_record_offset":7248,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2893,"record_offset":8408,"parent_record_offset":8152,"ancestor_record_offset":7248,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2893,"record_offset":8212,"parent_record_offset":8152,"ancestor_record_offset":7248,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2893,"record_offset":11844,"parent_record_offset":11816,"ancestor_record_offset":7248,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2893,"record_offset":11868,"parent_record_offset":11844,"ancestor_record_offset":7248,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":2893,"record_offset":15848,"parent_record_offset":15824,"ancestor_record_offset":7248,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":2893,"record_offset":28088,"parent_record_offset":28052,"ancestor_record_offset":7248,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2893,"record_offset":28052,"parent_record_offset":28024,"ancestor_record_offset":7248,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2893,"record_offset":28132,"parent_record_offset":28052,"ancestor_record_offset":7248,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":23360,"parent_record_offset":23312,"ancestor_record_offset":7248,"inlinee":11529,"source_line":35,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":27252,"parent_record_offset":27224,"ancestor_record_offset":7248,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":7248,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":2893,"record_offset":27224,"parent_record_offset":27196,"ancestor_record_offset":7248,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2893,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":7248,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2893,"record_offset":12420,"parent_record_offset":12392,"ancestor_record_offset":7248,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2893,"record_offset":12452,"parent_record_offset":12420,"ancestor_record_offset":7248,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2893,"record_offset":12392,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2893,"record_offset":12252,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2893,"record_offset":12804,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2893,"record_offset":12336,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2893,"record_offset":12196,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2893,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":7248,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2893,"record_offset":18592,"parent_record_offset":18552,"ancestor_record_offset":7248,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2893,"record_offset":12032,"parent_record_offset":12004,"ancestor_record_offset":7248,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2893,"record_offset":12660,"parent_record_offset":12552,"ancestor_record_offset":7248,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2893,"record_offset":12064,"parent_record_offset":12032,"ancestor_record_offset":7248,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":7248,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12896,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":7248,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":7248,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":18652,"parent_record_offset":18592,"ancestor_record_offset":7248,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12092,"parent_record_offset":12032,"ancestor_record_offset":7248,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":7248,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12224,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2893,"record_offset":12552,"parent_record_offset":12140,"ancestor_record_offset":7248,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":8992,"parent_record_offset":8956,"ancestor_record_offset":7248,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":10804,"parent_record_offset":10772,"ancestor_record_offset":7248,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":11256,"parent_record_offset":11052,"ancestor_record_offset":7248,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2893,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":7248,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":9204,"parent_record_offset":9064,"ancestor_record_offset":7248,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2893,"record_offset":9408,"parent_record_offset":9204,"ancestor_record_offset":7248,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2893,"record_offset":18552,"parent_record_offset":18516,"ancestor_record_offset":7248,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2893,"record_offset":12004,"parent_record_offset":11940,"ancestor_record_offset":7248,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2893,"record_offset":18516,"parent_record_offset":18456,"ancestor_record_offset":7248,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2893,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":7248,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":2893,"record_offset":15824,"parent_record_offset":15800,"ancestor_record_offset":7248,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2893,"record_offset":9064,"parent_record_offset":9028,"ancestor_record_offset":7248,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2893,"record_offset":9028,"parent_record_offset":8992,"ancestor_record_offset":7248,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":2893,"record_offset":15800,"parent_record_offset":15744,"ancestor_record_offset":7248,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":7248,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":13008,"parent_record_offset":12980,"ancestor_record_offset":7248,"inlinee":22043,"source_line":314,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":18456,"parent_record_offset":18424,"ancestor_record_offset":7248,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":11940,"parent_record_offset":11904,"ancestor_record_offset":7248,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2893,"record_offset":13032,"parent_record_offset":13008,"ancestor_record_offset":7248,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2893,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":7248,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2893,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":7248,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":9980,"parent_record_offset":9936,"ancestor_record_offset":7248,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":7248,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":17056,"parent_record_offset":17004,"ancestor_record_offset":7248,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":7248,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":18332,"parent_record_offset":18304,"ancestor_record_offset":7248,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2893,"record_offset":10320,"parent_record_offset":10292,"ancestor_record_offset":7248,"inlinee":111798,"source_line":97,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2893,"record_offset":10440,"parent_record_offset":10344,"ancestor_record_offset":7248,"inlinee":111800,"source_line":97,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2893,"record_offset":12980,"parent_record_offset":11904,"ancestor_record_offset":7248,"inlinee":111552,"source_line":105,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":7248,"inlinee":111424,"source_line":229,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::thumbnail_scroll::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2893,"record_offset":13104,"parent_record_offset":13072,"ancestor_record_offset":7248,"inlinee":111423,"source_line":234,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":2893,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":7248,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B6
// 首条位置: {"module":2893,"inlinee":111798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::position
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":10344,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":6398,"source_line":377,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":10320,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":10320,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2893,"record_offset":10404,"parent_record_offset":10376,"ancestor_record_offset":10320,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2893,"record_offset":10376,"parent_record_offset":10344,"ancestor_record_offset":10320,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":10320,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2893,"record_offset":10440,"parent_record_offset":10344,"ancestor_record_offset":10320,"inlinee":111800,"source_line":97,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":2893,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":10320,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4B8
// 首条位置: {"module":2893,"inlinee":111800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":10440,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":10440,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":2893,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":10440,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":2893,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":10440,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B3C0
// 首条位置: {"module":2875,"inlinee":111552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":13008,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":22043,"source_line":314,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2893,"record_offset":13032,"parent_record_offset":13008,"ancestor_record_offset":12980,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11220
// 首条位置: {"module":1239,"inlinee":70176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":77252,"parent_record_offset":77220,"ancestor_record_offset":77220,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74636,"parent_record_offset":74392,"ancestor_record_offset":74392,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74696,"parent_record_offset":74392,"ancestor_record_offset":74392,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74432,"parent_record_offset":74392,"ancestor_record_offset":74392,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74756,"parent_record_offset":74392,"ancestor_record_offset":74392,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":75540,"parent_record_offset":75516,"ancestor_record_offset":74392,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":75108,"parent_record_offset":75084,"ancestor_record_offset":74392,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":75192,"parent_record_offset":75168,"ancestor_record_offset":74392,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":75608,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":75340,"parent_record_offset":75312,"ancestor_record_offset":74392,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":75312,"parent_record_offset":75284,"ancestor_record_offset":74392,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":77356,"parent_record_offset":77332,"ancestor_record_offset":77220,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":75132,"parent_record_offset":75108,"ancestor_record_offset":74392,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":75456,"parent_record_offset":75432,"ancestor_record_offset":74392,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":74808,"parent_record_offset":74780,"ancestor_record_offset":74392,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":75564,"parent_record_offset":75540,"ancestor_record_offset":74392,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":75252,"parent_record_offset":75224,"ancestor_record_offset":74392,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":75284,"parent_record_offset":75252,"ancestor_record_offset":74392,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":75224,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":75084,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":75636,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":75168,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":75028,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":75516,"parent_record_offset":75492,"ancestor_record_offset":74392,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":74864,"parent_record_offset":74836,"ancestor_record_offset":74392,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":75492,"parent_record_offset":75384,"ancestor_record_offset":74392,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":74896,"parent_record_offset":74864,"ancestor_record_offset":74392,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":75432,"parent_record_offset":75408,"ancestor_record_offset":74392,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":75728,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":75408,"parent_record_offset":75384,"ancestor_record_offset":74392,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":74948,"parent_record_offset":74924,"ancestor_record_offset":74392,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":74924,"parent_record_offset":74864,"ancestor_record_offset":74392,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":74972,"parent_record_offset":74948,"ancestor_record_offset":74392,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":75056,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":75384,"parent_record_offset":74972,"ancestor_record_offset":74392,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":74836,"parent_record_offset":74780,"ancestor_record_offset":74392,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74544,"parent_record_offset":74484,"ancestor_record_offset":74392,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74484,"parent_record_offset":74432,"ancestor_record_offset":74392,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74780,"parent_record_offset":74756,"ancestor_record_offset":74392,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":77276,"parent_record_offset":77252,"ancestor_record_offset":77220,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74456,"parent_record_offset":74432,"ancestor_record_offset":74392,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74516,"parent_record_offset":74484,"ancestor_record_offset":74392,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":74568,"parent_record_offset":74544,"ancestor_record_offset":74392,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74664,"parent_record_offset":74636,"ancestor_record_offset":74392,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":74600,"parent_record_offset":74484,"ancestor_record_offset":74392,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1121D
// 首条位置: {"module":1239,"inlinee":70173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":66292,"parent_record_offset":66260,"ancestor_record_offset":66260,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63676,"parent_record_offset":63432,"ancestor_record_offset":63432,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63736,"parent_record_offset":63432,"ancestor_record_offset":63432,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63472,"parent_record_offset":63432,"ancestor_record_offset":63432,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63796,"parent_record_offset":63432,"ancestor_record_offset":63432,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":64580,"parent_record_offset":64556,"ancestor_record_offset":63432,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":64148,"parent_record_offset":64124,"ancestor_record_offset":63432,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":64232,"parent_record_offset":64208,"ancestor_record_offset":63432,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":64648,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":64380,"parent_record_offset":64352,"ancestor_record_offset":63432,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":64352,"parent_record_offset":64324,"ancestor_record_offset":63432,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":66396,"parent_record_offset":66372,"ancestor_record_offset":66260,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":64172,"parent_record_offset":64148,"ancestor_record_offset":63432,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":64496,"parent_record_offset":64472,"ancestor_record_offset":63432,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":63848,"parent_record_offset":63820,"ancestor_record_offset":63432,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":64604,"parent_record_offset":64580,"ancestor_record_offset":63432,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":64292,"parent_record_offset":64264,"ancestor_record_offset":63432,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":64324,"parent_record_offset":64292,"ancestor_record_offset":63432,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":64264,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":64124,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":64676,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":64208,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":64068,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":64556,"parent_record_offset":64532,"ancestor_record_offset":63432,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":63904,"parent_record_offset":63876,"ancestor_record_offset":63432,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":64532,"parent_record_offset":64424,"ancestor_record_offset":63432,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":63936,"parent_record_offset":63904,"ancestor_record_offset":63432,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64472,"parent_record_offset":64448,"ancestor_record_offset":63432,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64768,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64448,"parent_record_offset":64424,"ancestor_record_offset":63432,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":63988,"parent_record_offset":63964,"ancestor_record_offset":63432,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":63964,"parent_record_offset":63904,"ancestor_record_offset":63432,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64012,"parent_record_offset":63988,"ancestor_record_offset":63432,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64096,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":64424,"parent_record_offset":64012,"ancestor_record_offset":63432,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":63876,"parent_record_offset":63820,"ancestor_record_offset":63432,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63584,"parent_record_offset":63524,"ancestor_record_offset":63432,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63524,"parent_record_offset":63472,"ancestor_record_offset":63432,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63820,"parent_record_offset":63796,"ancestor_record_offset":63432,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":66316,"parent_record_offset":66292,"ancestor_record_offset":66260,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63496,"parent_record_offset":63472,"ancestor_record_offset":63432,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63556,"parent_record_offset":63524,"ancestor_record_offset":63432,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":63608,"parent_record_offset":63584,"ancestor_record_offset":63432,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63704,"parent_record_offset":63676,"ancestor_record_offset":63432,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":63640,"parent_record_offset":63524,"ancestor_record_offset":63432,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4BB
// 首条位置: {"module":2893,"inlinee":111803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2893,"record_offset":23392,"parent_record_offset":23312,"ancestor_record_offset":23312,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2893,"record_offset":23476,"parent_record_offset":23312,"ancestor_record_offset":23312,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2893,"record_offset":23360,"parent_record_offset":23312,"ancestor_record_offset":23312,"inlinee":11529,"source_line":35,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":23312,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":23312,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":24004,"parent_record_offset":23980,"ancestor_record_offset":23312,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":23932,"parent_record_offset":23784,"ancestor_record_offset":23312,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2893,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":23312,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":23312,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23784,"parent_record_offset":23752,"ancestor_record_offset":23312,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23752,"parent_record_offset":23724,"ancestor_record_offset":23312,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":23820,"parent_record_offset":23784,"ancestor_record_offset":23312,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23604,"parent_record_offset":23560,"ancestor_record_offset":23312,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23632,"parent_record_offset":23604,"ancestor_record_offset":23312,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23660,"parent_record_offset":23632,"ancestor_record_offset":23312,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2893,"record_offset":23560,"parent_record_offset":23532,"ancestor_record_offset":23312,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2893,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":23312,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2893,"record_offset":23504,"parent_record_offset":23476,"ancestor_record_offset":23312,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":23696,"parent_record_offset":23660,"ancestor_record_offset":23312,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2893,"record_offset":23844,"parent_record_offset":23820,"ancestor_record_offset":23312,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2893,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":23312,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2893,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":23312,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2893,"record_offset":23416,"parent_record_offset":23392,"ancestor_record_offset":23312,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2893,"record_offset":24060,"parent_record_offset":23660,"ancestor_record_offset":23312,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":23312,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 410
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11238
// 首条位置: {"module":1239,"inlinee":70200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":157896,"parent_record_offset":157872,"ancestor_record_offset":157872,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":158004,"parent_record_offset":157948,"ancestor_record_offset":157872,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":157948,"parent_record_offset":157896,"ancestor_record_offset":157872,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":157920,"parent_record_offset":157896,"ancestor_record_offset":157872,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":157976,"parent_record_offset":157948,"ancestor_record_offset":157872,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":158028,"parent_record_offset":158004,"ancestor_record_offset":157872,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":158412,"parent_record_offset":158324,"ancestor_record_offset":158300,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":158380,"parent_record_offset":158352,"ancestor_record_offset":158300,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21721
// 首条位置: {"module":4247,"inlinee":136993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::time::impl$34::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":4247,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":31569,"source_line":248,"archive":"_pdb/C/modules/04247.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::SystemTime::intervals
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":4247,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":744,"inlinee":20849,"source_line":84,"archive":"_pdb/C/modules/04247.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$5::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":4247,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":744,"inlinee":31549,"source_line":129,"archive":"_pdb/C/modules/04247.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$9
// 记录形式: compiled_procedure; instances=1
// 源码行: 433
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":229772,"rva":18479760,"va":5387188880}
// 编译实例 RVAs: 0x119fa90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2487986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e84e09daff","size":5,"soft_fail":false,"target_va":5384703536,"va":5387191517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2534302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e86254d9ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5387190537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2541079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8e939d9ff","size":5,"soft_fail":false,"target_va":5384653152,"va":5387194226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2534895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e81152d9ff","size":5,"soft_fail":false,"target_va":5384657024,"va":5387191914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2536786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8ae4ad9ff","size":5,"soft_fail":false,"target_va":5384656512,"va":5387193293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2425013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e84bffdaff","size":5,"soft_fail":false,"target_va":5384768864,"va":5387193872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2545827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e85d27d9ff","size":5,"soft_fail":false,"target_va":5384648032,"va":5387193854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2581578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8b69bd8ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5387190981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11512346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8e65550ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387191029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=33
//     evidence={"assembly":"\tcallq\t-11511879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8b95750ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387190578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18468823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e82930e6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387191010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17852538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e88697effe","size":5,"soft_fail":false,"target_va":5369338848,"va":5387191381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17751394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e89e22f1fe","size":5,"soft_fail":false,"target_va":5369439344,"va":5387190733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=6
//     target_aliases=6 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13709957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e87bcd2eff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387188992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13705132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e854e02eff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387189511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13704755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8cde12eff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387189534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10953361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e86fdd58ff","size":5,"soft_fail":false,"target_va":5376236864,"va":5387190220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6924126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8a25896ff","size":5,"soft_fail":false,"target_va":5380267296,"va":5387191417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b08cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6911104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8808b96ff","size":5,"soft_fail":false,"target_va":5380279536,"va":5387190635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6125305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e80789a2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387189476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5586117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e83bc3aaff","size":5,"soft_fail":false,"target_va":5381607664,"va":5387193776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-169498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8e669fdff","size":5,"soft_fail":false,"target_va":5387020736,"va":5387190229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-172052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8ec5ffdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387195599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-170082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e89e67fdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387195533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-149367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e889b8fdff","size":5,"soft_fail":false,"target_va":5387045984,"va":5387195346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141189950
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8528ffeff","size":5,"soft_fail":false,"target_va":5387098448,"va":5387192825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8f47ffeff","size":5,"soft_fail":false,"target_va":5387100208,"va":5387198519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141191450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-65466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e84600ffff","size":5,"soft_fail":false,"target_va":5387129936,"va":5387195397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411953c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-49197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8d33fffff","size":5,"soft_fail":false,"target_va":5387146176,"va":5387195368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e86f230000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387195612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8a7270000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387195556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b48210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10123901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e87d7a9a00","size":5,"soft_fail":false,"target_va":5397316112,"va":5387192206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423457b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18504479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e81f5b1a01","size":5,"soft_fail":false,"target_va":5405693872,"va":5387189388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142349eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18520481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8a1991a01","size":5,"soft_fail":false,"target_va":5405712048,"va":5387191562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14234a730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18521659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e83b9e1a01","size":5,"soft_fail":false,"target_va":5405714224,"va":5387192560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29156757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e895e5bc01","size":5,"soft_fail":false,"target_va":5416351392,"va":5387194630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29862697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e829abc701","size":5,"soft_fail":false,"target_va":5417057040,"va":5387194338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29866355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e873b9c701","size":5,"soft_fail":false,"target_va":5417057056,"va":5387190696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2396314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8666fdbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387191173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29425010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e872fdc001","size":5,"soft_fail":false,"target_va":5416620794,"va":5387195779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2566573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e853d6d8ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5387190648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t119858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e832d40100","size":5,"soft_fail":false,"target_va":5387312896,"va":5387193033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t70842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8ba140100","size":5,"soft_fail":false,"target_va":5387265328,"va":5387194481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29425772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e86c00c101","size":5,"soft_fail":false,"target_va":5416621536,"va":5387195759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29425897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8e900c101","size":5,"soft_fail":false,"target_va":5416621600,"va":5387195698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::session::SessionMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-81958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8dabffeff","size":5,"soft_fail":false,"target_va":5387114704,"va":5387196657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::session::NextSessionKind> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-82902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e82abcfeff","size":5,"soft_fail":false,"target_va":5387114928,"va":5387197825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29425675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e80b00c101","size":5,"soft_fail":false,"target_va":5416621632,"va":5387195952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2363115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e815f1dbff","size":5,"soft_fail":false,"target_va":5384830080,"va":5387193190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6929545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8774396ff","size":5,"soft_fail":false,"target_va":5380261184,"va":5387190724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6923324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8c45b96ff","size":5,"soft_fail":false,"target_va":5380267776,"va":5387191095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6923183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8515c96ff","size":5,"soft_fail":false,"target_va":5380267312,"va":5387190490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::assets_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6928601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8274796ff","size":5,"soft_fail":false,"target_va":5380262112,"va":5387190708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_game::parse_saved_game_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6809419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8b51898ff","size":5,"soft_fail":false,"target_va":5380381344,"va":5387190758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6479130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e81add6200","size":5,"soft_fail":false,"target_va":5393670960,"va":5387191825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2469053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e84353daff","size":5,"soft_fail":false,"target_va":5384724448,"va":5387193496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10988231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8395558ff","size":5,"soft_fail":false,"target_va":5376206368,"va":5387194594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10981194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8b67058ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387189557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::load_session_meta
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18505716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8f45f1a01","size":5,"soft_fail":false,"target_va":5405696336,"va":5387190615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3891066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e886a0c4ff","size":5,"soft_fail":false,"target_va":5383300064,"va":5387191125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::local_time::system_time_to_local_time
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3907708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188880,"raw_hex":"e8845fc4ff","size":5,"soft_fail":false,"target_va":5383281632,"va":5387189335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":236364,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":243184,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_state","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.17.3/src/state/resources.rs
//     evidence={"module":1239,"record_offset":243244,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":10613,"source_line":134,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":248952,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":245636,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":235412,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1239,"record_offset":236732,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":245252,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":241624,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":234860,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":242760,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":231308,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":235228,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5420,"source_line":1265,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1239,"record_offset":237736,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1239,"record_offset":237820,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1239,"record_offset":231224,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1239,"record_offset":231112,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1239,"record_offset":241780,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$35::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1239,"record_offset":240452,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":70225,"source_line":325,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":235468,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":235328,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1239,"record_offset":241660,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":242704,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":234832,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":235580,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":245280,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":1239,"record_offset":231280,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":245836,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":10269,"source_line":505,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":234748,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231384,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::hover
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":232568,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21726,"source_line":388,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231528,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::mei
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":245780,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21689,"source_line":384,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":246152,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::set_default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":230048,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":22027,"source_line":1544,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":249396,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":233600,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21795,"source_line":398,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":249192,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":233660,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::display_save_file_load_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":249128,"parent_record_offset":229772,"ancestor_record_offset":229772,"inlinee":70227,"source_line":676,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":236176,"parent_record_offset":236152,"ancestor_record_offset":229772,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":236200,"parent_record_offset":236176,"ancestor_record_offset":229772,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":235108,"parent_record_offset":235084,"ancestor_record_offset":229772,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":235132,"parent_record_offset":235108,"ancestor_record_offset":229772,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":236224,"parent_record_offset":236200,"ancestor_record_offset":229772,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":235156,"parent_record_offset":235132,"ancestor_record_offset":229772,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":236152,"parent_record_offset":235996,"ancestor_record_offset":229772,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":235084,"parent_record_offset":235028,"ancestor_record_offset":229772,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1239,"record_offset":240164,"parent_record_offset":240136,"ancestor_record_offset":229772,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":236412,"parent_record_offset":236388,"ancestor_record_offset":229772,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":246020,"parent_record_offset":245988,"ancestor_record_offset":229772,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235920,"parent_record_offset":235884,"ancestor_record_offset":229772,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235056,"parent_record_offset":235028,"ancestor_record_offset":229772,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235028,"parent_record_offset":235004,"ancestor_record_offset":229772,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":246068,"parent_record_offset":246044,"ancestor_record_offset":229772,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":246044,"parent_record_offset":246020,"ancestor_record_offset":229772,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235996,"parent_record_offset":235956,"ancestor_record_offset":229772,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235956,"parent_record_offset":235920,"ancestor_record_offset":229772,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":235004,"parent_record_offset":234980,"ancestor_record_offset":229772,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":236040,"parent_record_offset":235996,"ancestor_record_offset":229772,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":235844,"parent_record_offset":235772,"ancestor_record_offset":229772,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":245988,"parent_record_offset":245952,"ancestor_record_offset":229772,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":235816,"parent_record_offset":235772,"ancestor_record_offset":229772,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":245928,"parent_record_offset":245860,"ancestor_record_offset":229772,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":245952,"parent_record_offset":245928,"ancestor_record_offset":229772,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":235884,"parent_record_offset":235844,"ancestor_record_offset":229772,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":235736,"parent_record_offset":235700,"ancestor_record_offset":229772,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":235772,"parent_record_offset":235736,"ancestor_record_offset":229772,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1239,"record_offset":235700,"parent_record_offset":235660,"ancestor_record_offset":229772,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":243212,"parent_record_offset":243184,"ancestor_record_offset":229772,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1239,"record_offset":236064,"parent_record_offset":236040,"ancestor_record_offset":229772,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":238120,"parent_record_offset":237968,"ancestor_record_offset":229772,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":248980,"parent_record_offset":248952,"ancestor_record_offset":229772,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":230812,"parent_record_offset":230788,"ancestor_record_offset":229772,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1239,"record_offset":238716,"parent_record_offset":238632,"ancestor_record_offset":229772,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":230380,"parent_record_offset":230356,"ancestor_record_offset":229772,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":230464,"parent_record_offset":230440,"ancestor_record_offset":229772,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":230880,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":236112,"parent_record_offset":236088,"ancestor_record_offset":229772,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1239,"record_offset":245568,"parent_record_offset":245540,"ancestor_record_offset":229772,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1239,"record_offset":245540,"parent_record_offset":245304,"ancestor_record_offset":229772,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1239,"record_offset":236088,"parent_record_offset":236064,"ancestor_record_offset":229772,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":230612,"parent_record_offset":230584,"ancestor_record_offset":229772,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":230584,"parent_record_offset":230556,"ancestor_record_offset":229772,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":245396,"parent_record_offset":245372,"ancestor_record_offset":229772,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":235436,"parent_record_offset":235412,"ancestor_record_offset":229772,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":236388,"parent_record_offset":236364,"ancestor_record_offset":229772,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":231080,"parent_record_offset":230048,"ancestor_record_offset":229772,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1239,"record_offset":236280,"parent_record_offset":235844,"ancestor_record_offset":229772,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":230404,"parent_record_offset":230380,"ancestor_record_offset":229772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=148
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":231352,"parent_record_offset":231308,"ancestor_record_offset":229772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":231184,"parent_record_offset":231160,"ancestor_record_offset":229772,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":230728,"parent_record_offset":230704,"ancestor_record_offset":229772,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":230084,"parent_record_offset":230048,"ancestor_record_offset":229772,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":232600,"parent_record_offset":232568,"ancestor_record_offset":229772,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":231248,"parent_record_offset":231224,"ancestor_record_offset":229772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":246108,"parent_record_offset":245952,"ancestor_record_offset":229772,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":237896,"parent_record_offset":237856,"ancestor_record_offset":229772,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1239,"record_offset":241876,"parent_record_offset":241852,"ancestor_record_offset":229772,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1239,"record_offset":241900,"parent_record_offset":241876,"ancestor_record_offset":229772,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1239,"record_offset":231136,"parent_record_offset":231112,"ancestor_record_offset":229772,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1239,"record_offset":241828,"parent_record_offset":241804,"ancestor_record_offset":229772,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1239,"record_offset":231160,"parent_record_offset":231136,"ancestor_record_offset":229772,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1239,"record_offset":238780,"parent_record_offset":237820,"ancestor_record_offset":229772,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1239,"record_offset":241804,"parent_record_offset":241780,"ancestor_record_offset":229772,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":235352,"parent_record_offset":235328,"ancestor_record_offset":229772,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":235376,"parent_record_offset":235352,"ancestor_record_offset":229772,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1239,"record_offset":245740,"parent_record_offset":245716,"ancestor_record_offset":229772,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1239,"record_offset":241708,"parent_record_offset":241684,"ancestor_record_offset":229772,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1239,"record_offset":238316,"parent_record_offset":237968,"ancestor_record_offset":229772,"inlinee":34458,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":230836,"parent_record_offset":230812,"ancestor_record_offset":229772,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":230524,"parent_record_offset":230496,"ancestor_record_offset":229772,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":230556,"parent_record_offset":230524,"ancestor_record_offset":229772,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":230496,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":230356,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":230908,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":230440,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":230300,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":230788,"parent_record_offset":230764,"ancestor_record_offset":229772,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":232652,"parent_record_offset":232628,"ancestor_record_offset":229772,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":230140,"parent_record_offset":230112,"ancestor_record_offset":229772,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":230764,"parent_record_offset":230656,"ancestor_record_offset":229772,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":230172,"parent_record_offset":230140,"ancestor_record_offset":229772,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230704,"parent_record_offset":230680,"ancestor_record_offset":229772,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":231000,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230680,"parent_record_offset":230656,"ancestor_record_offset":229772,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230224,"parent_record_offset":230200,"ancestor_record_offset":229772,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":232712,"parent_record_offset":232652,"ancestor_record_offset":229772,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230200,"parent_record_offset":230140,"ancestor_record_offset":229772,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230248,"parent_record_offset":230224,"ancestor_record_offset":229772,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230328,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":230656,"parent_record_offset":230248,"ancestor_record_offset":229772,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":235516,"parent_record_offset":235492,"ancestor_record_offset":229772,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":238176,"parent_record_offset":237968,"ancestor_record_offset":229772,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":235492,"parent_record_offset":235468,"ancestor_record_offset":229772,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":235540,"parent_record_offset":235516,"ancestor_record_offset":229772,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238036,"parent_record_offset":237968,"ancestor_record_offset":229772,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":232628,"parent_record_offset":232568,"ancestor_record_offset":229772,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":230112,"parent_record_offset":230048,"ancestor_record_offset":229772,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":233752,"parent_record_offset":233692,"ancestor_record_offset":229772,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1239,"record_offset":235620,"parent_record_offset":235580,"ancestor_record_offset":229772,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238632,"parent_record_offset":237856,"ancestor_record_offset":229772,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238656,"parent_record_offset":238632,"ancestor_record_offset":229772,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238744,"parent_record_offset":238632,"ancestor_record_offset":229772,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":245372,"parent_record_offset":245304,"ancestor_record_offset":229772,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":245344,"parent_record_offset":245304,"ancestor_record_offset":229772,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1239,"record_offset":245304,"parent_record_offset":245280,"ancestor_record_offset":229772,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1239,"record_offset":238684,"parent_record_offset":238656,"ancestor_record_offset":229772,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1239,"record_offset":241684,"parent_record_offset":241660,"ancestor_record_offset":229772,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1239,"record_offset":235660,"parent_record_offset":235620,"ancestor_record_offset":229772,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":1239,"record_offset":245716,"parent_record_offset":245692,"ancestor_record_offset":229772,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":1239,"record_offset":245692,"parent_record_offset":245636,"ancestor_record_offset":229772,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":1239,"record_offset":249160,"parent_record_offset":249128,"ancestor_record_offset":229772,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":245860,"parent_record_offset":245836,"ancestor_record_offset":229772,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":248864,"parent_record_offset":247864,"ancestor_record_offset":229772,"inlinee":21727,"source_line":350,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":249304,"parent_record_offset":249244,"ancestor_record_offset":229772,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":249244,"parent_record_offset":249192,"ancestor_record_offset":229772,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":233692,"parent_record_offset":233660,"ancestor_record_offset":229772,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":231052,"parent_record_offset":230048,"ancestor_record_offset":229772,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":248888,"parent_record_offset":248864,"ancestor_record_offset":229772,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":234772,"parent_record_offset":234748,"ancestor_record_offset":229772,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":234796,"parent_record_offset":234772,"ancestor_record_offset":229772,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231460,"parent_record_offset":231432,"ancestor_record_offset":229772,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231432,"parent_record_offset":231408,"ancestor_record_offset":229772,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231492,"parent_record_offset":231408,"ancestor_record_offset":229772,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":231408,"parent_record_offset":231384,"ancestor_record_offset":229772,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":249420,"parent_record_offset":249396,"ancestor_record_offset":229772,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":233628,"parent_record_offset":233600,"ancestor_record_offset":229772,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":249360,"parent_record_offset":249244,"ancestor_record_offset":229772,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$9::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":237856,"parent_record_offset":237820,"ancestor_record_offset":229772,"inlinee":70221,"source_line":477,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$9::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":237968,"parent_record_offset":237896,"ancestor_record_offset":229772,"inlinee":70223,"source_line":484,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$9::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1124D
// 首条位置: {"module":1239,"inlinee":70221}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1239,"record_offset":237896,"parent_record_offset":237856,"ancestor_record_offset":237856,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238632,"parent_record_offset":237856,"ancestor_record_offset":237856,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":238120,"parent_record_offset":237968,"ancestor_record_offset":237856,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1239,"record_offset":238716,"parent_record_offset":238632,"ancestor_record_offset":237856,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":238144,"parent_record_offset":238120,"ancestor_record_offset":237856,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":237940,"parent_record_offset":237896,"ancestor_record_offset":237856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":238060,"parent_record_offset":238036,"ancestor_record_offset":237856,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":238084,"parent_record_offset":238060,"ancestor_record_offset":237856,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1239,"record_offset":238316,"parent_record_offset":237968,"ancestor_record_offset":237856,"inlinee":34458,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238224,"parent_record_offset":238200,"ancestor_record_offset":237856,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":238176,"parent_record_offset":237968,"ancestor_record_offset":237856,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":238200,"parent_record_offset":238176,"ancestor_record_offset":237856,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238248,"parent_record_offset":238224,"ancestor_record_offset":237856,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238036,"parent_record_offset":237968,"ancestor_record_offset":237856,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238656,"parent_record_offset":238632,"ancestor_record_offset":237856,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1239,"record_offset":238744,"parent_record_offset":238632,"ancestor_record_offset":237856,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1239,"record_offset":238684,"parent_record_offset":238656,"ancestor_record_offset":237856,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$9::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1239,"record_offset":237968,"parent_record_offset":237896,"ancestor_record_offset":237856,"inlinee":70223,"source_line":484,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$9::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1124F
// 首条位置: {"module":1239,"inlinee":70223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":238120,"parent_record_offset":237968,"ancestor_record_offset":237968,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":238288,"parent_record_offset":237968,"ancestor_record_offset":237968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1239,"record_offset":238316,"parent_record_offset":237968,"ancestor_record_offset":237968,"inlinee":34458,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":238176,"parent_record_offset":237968,"ancestor_record_offset":237968,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238036,"parent_record_offset":237968,"ancestor_record_offset":237968,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":238144,"parent_record_offset":238120,"ancestor_record_offset":237968,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":238060,"parent_record_offset":238036,"ancestor_record_offset":237968,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1239,"record_offset":238084,"parent_record_offset":238060,"ancestor_record_offset":237968,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238224,"parent_record_offset":238200,"ancestor_record_offset":237968,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1239,"record_offset":238200,"parent_record_offset":238176,"ancestor_record_offset":237968,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1239,"record_offset":238248,"parent_record_offset":238224,"ancestor_record_offset":237968,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$10
// 记录形式: compiled_procedure; instances=1
// 源码行: 620
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":227156,"rva":18478960,"va":5387188080}
// 编译实例 RVAs: 0x119f770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13709204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e86cd02eff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387188239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13703788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e894e52eff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387188167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13703408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e810e72eff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387188187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10951673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e807e458ff","size":5,"soft_fail":false,"target_va":5376236864,"va":5387188532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6123967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e8418ea2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387188138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-167809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e87f70fdff","size":5,"soft_fail":false,"target_va":5387020736,"va":5387188540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141179590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-156546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e87e9cfdff","size":5,"soft_fail":false,"target_va":5387031952,"va":5387188493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29432675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e8631bc101","size":5,"soft_fail":false,"target_va":5416621536,"va":5387188856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29432727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e8971bc101","size":5,"soft_fail":false,"target_va":5416621600,"va":5387188868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4053795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e8dd24c2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387188510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10979840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387188080,"raw_hex":"e8007658ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387188203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":229684,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227296,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227436,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":228480,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":21795,"source_line":398,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":228540,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":21798,"source_line":357,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":228600,"parent_record_offset":227156,"ancestor_record_offset":227156,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":228208,"parent_record_offset":228184,"ancestor_record_offset":227156,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":227776,"parent_record_offset":227752,"ancestor_record_offset":227156,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":227860,"parent_record_offset":227836,"ancestor_record_offset":227156,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":228276,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":228008,"parent_record_offset":227980,"ancestor_record_offset":227156,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":227980,"parent_record_offset":227952,"ancestor_record_offset":227156,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":228448,"parent_record_offset":227436,"ancestor_record_offset":227156,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":227800,"parent_record_offset":227776,"ancestor_record_offset":227156,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":228124,"parent_record_offset":228100,"ancestor_record_offset":227156,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":227476,"parent_record_offset":227436,"ancestor_record_offset":227156,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":228656,"parent_record_offset":228628,"ancestor_record_offset":227156,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":228232,"parent_record_offset":228208,"ancestor_record_offset":227156,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":227920,"parent_record_offset":227892,"ancestor_record_offset":227156,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":227952,"parent_record_offset":227920,"ancestor_record_offset":227156,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":227892,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":227752,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":228304,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":227836,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":227696,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":228184,"parent_record_offset":228160,"ancestor_record_offset":227156,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":228756,"parent_record_offset":228716,"ancestor_record_offset":227156,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":227532,"parent_record_offset":227504,"ancestor_record_offset":227156,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":228160,"parent_record_offset":228052,"ancestor_record_offset":227156,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":227564,"parent_record_offset":227532,"ancestor_record_offset":227156,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":228100,"parent_record_offset":228076,"ancestor_record_offset":227156,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":228396,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":228076,"parent_record_offset":228052,"ancestor_record_offset":227156,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":227616,"parent_record_offset":227592,"ancestor_record_offset":227156,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":228816,"parent_record_offset":228756,"ancestor_record_offset":227156,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":227592,"parent_record_offset":227532,"ancestor_record_offset":227156,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":227640,"parent_record_offset":227616,"ancestor_record_offset":227156,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":227724,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":228052,"parent_record_offset":227640,"ancestor_record_offset":227156,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":228716,"parent_record_offset":228684,"ancestor_record_offset":227156,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":227504,"parent_record_offset":227436,"ancestor_record_offset":227156,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":228684,"parent_record_offset":228628,"ancestor_record_offset":227156,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":228628,"parent_record_offset":228600,"ancestor_record_offset":227156,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":229708,"parent_record_offset":229684,"ancestor_record_offset":227156,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":229732,"parent_record_offset":229708,"ancestor_record_offset":227156,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227368,"parent_record_offset":227344,"ancestor_record_offset":227156,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227344,"parent_record_offset":227320,"ancestor_record_offset":227156,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227400,"parent_record_offset":227320,"ancestor_record_offset":227156,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":227320,"parent_record_offset":227296,"ancestor_record_offset":227156,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":228508,"parent_record_offset":228480,"ancestor_record_offset":227156,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$10::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 628
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111CA
// 首条位置: {"module":1235,"inlinee":70090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1235,"record_offset":128168,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":128632,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128084,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128456,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128516,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128576,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":126964,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21795,"source_line":398,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128252,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":127024,"parent_record_offset":126888,"ancestor_record_offset":126888,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":128808,"parent_record_offset":128784,"ancestor_record_offset":126888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":128832,"parent_record_offset":128808,"ancestor_record_offset":126888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":128856,"parent_record_offset":128832,"ancestor_record_offset":126888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":128784,"parent_record_offset":128728,"ancestor_record_offset":126888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1235,"record_offset":128216,"parent_record_offset":128192,"ancestor_record_offset":126888,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":128756,"parent_record_offset":128728,"ancestor_record_offset":126888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":128728,"parent_record_offset":128704,"ancestor_record_offset":126888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":128704,"parent_record_offset":128680,"ancestor_record_offset":126888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":127832,"parent_record_offset":127808,"ancestor_record_offset":126888,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":127400,"parent_record_offset":127376,"ancestor_record_offset":126888,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":127484,"parent_record_offset":127460,"ancestor_record_offset":126888,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":127900,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":127632,"parent_record_offset":127604,"ancestor_record_offset":126888,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":127604,"parent_record_offset":127576,"ancestor_record_offset":126888,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":130104,"parent_record_offset":130080,"ancestor_record_offset":126888,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":128192,"parent_record_offset":128168,"ancestor_record_offset":126888,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":127424,"parent_record_offset":127400,"ancestor_record_offset":126888,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":128656,"parent_record_offset":128632,"ancestor_record_offset":126888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":127748,"parent_record_offset":127724,"ancestor_record_offset":126888,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":127072,"parent_record_offset":127048,"ancestor_record_offset":126888,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1235,"record_offset":127856,"parent_record_offset":127832,"ancestor_record_offset":126888,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":127544,"parent_record_offset":127516,"ancestor_record_offset":126888,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":127576,"parent_record_offset":127544,"ancestor_record_offset":126888,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":127516,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":127376,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":127928,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":127460,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1235,"record_offset":127320,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1235,"record_offset":127808,"parent_record_offset":127784,"ancestor_record_offset":126888,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":127156,"parent_record_offset":127128,"ancestor_record_offset":126888,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":127784,"parent_record_offset":127676,"ancestor_record_offset":126888,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":127188,"parent_record_offset":127156,"ancestor_record_offset":126888,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127724,"parent_record_offset":127700,"ancestor_record_offset":126888,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":128020,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127700,"parent_record_offset":127676,"ancestor_record_offset":126888,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127240,"parent_record_offset":127216,"ancestor_record_offset":126888,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127216,"parent_record_offset":127156,"ancestor_record_offset":126888,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127264,"parent_record_offset":127240,"ancestor_record_offset":126888,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127348,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":127676,"parent_record_offset":127264,"ancestor_record_offset":126888,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":127128,"parent_record_offset":127100,"ancestor_record_offset":126888,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":127100,"parent_record_offset":127048,"ancestor_record_offset":126888,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128364,"parent_record_offset":128304,"ancestor_record_offset":126888,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128304,"parent_record_offset":128252,"ancestor_record_offset":126888,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":127048,"parent_record_offset":127024,"ancestor_record_offset":126888,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":128108,"parent_record_offset":128084,"ancestor_record_offset":126888,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":128132,"parent_record_offset":128108,"ancestor_record_offset":126888,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128276,"parent_record_offset":128252,"ancestor_record_offset":126888,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128336,"parent_record_offset":128304,"ancestor_record_offset":126888,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":128388,"parent_record_offset":128364,"ancestor_record_offset":126888,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128484,"parent_record_offset":128456,"ancestor_record_offset":126888,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":126992,"parent_record_offset":126964,"ancestor_record_offset":126888,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":128420,"parent_record_offset":128304,"ancestor_record_offset":126888,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 663
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11215
// 首条位置: {"module":1239,"inlinee":70165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":45452,"parent_record_offset":45212,"ancestor_record_offset":45212,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":45368,"parent_record_offset":45212,"ancestor_record_offset":45212,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::hotkey
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":40560,"parent_record_offset":40512,"ancestor_record_offset":40512,"inlinee":21803,"source_line":1358,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::on_click
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":40620,"parent_record_offset":40512,"ancestor_record_offset":40512,"inlinee":70166,"source_line":1328,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$54::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":41784,"parent_record_offset":40512,"ancestor_record_offset":40512,"inlinee":21691,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":45480,"parent_record_offset":45452,"ancestor_record_offset":45212,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":41444,"parent_record_offset":41420,"ancestor_record_offset":40512,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":41012,"parent_record_offset":40988,"ancestor_record_offset":40512,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":41096,"parent_record_offset":41072,"ancestor_record_offset":40512,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":41512,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":41244,"parent_record_offset":41216,"ancestor_record_offset":40512,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":41216,"parent_record_offset":41188,"ancestor_record_offset":40512,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":45584,"parent_record_offset":45560,"ancestor_record_offset":45212,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":45328,"parent_record_offset":45304,"ancestor_record_offset":45212,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":41036,"parent_record_offset":41012,"ancestor_record_offset":40512,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":41360,"parent_record_offset":41336,"ancestor_record_offset":40512,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":40712,"parent_record_offset":40684,"ancestor_record_offset":40512,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":40512,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":41468,"parent_record_offset":41444,"ancestor_record_offset":40512,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":41156,"parent_record_offset":41128,"ancestor_record_offset":40512,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":41188,"parent_record_offset":41156,"ancestor_record_offset":40512,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":41128,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":40988,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":41540,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":41072,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":40932,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":41420,"parent_record_offset":41396,"ancestor_record_offset":40512,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":41916,"parent_record_offset":41888,"ancestor_record_offset":40512,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":40768,"parent_record_offset":40740,"ancestor_record_offset":40512,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":41396,"parent_record_offset":41288,"ancestor_record_offset":40512,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":40800,"parent_record_offset":40768,"ancestor_record_offset":40512,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":41336,"parent_record_offset":41312,"ancestor_record_offset":40512,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":41632,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":41312,"parent_record_offset":41288,"ancestor_record_offset":40512,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":40512,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":41976,"parent_record_offset":41916,"ancestor_record_offset":40512,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":40828,"parent_record_offset":40768,"ancestor_record_offset":40512,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":40876,"parent_record_offset":40852,"ancestor_record_offset":40512,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":40960,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":41288,"parent_record_offset":40876,"ancestor_record_offset":40512,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":41888,"parent_record_offset":41860,"ancestor_record_offset":40512,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":40740,"parent_record_offset":40684,"ancestor_record_offset":40512,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":41860,"parent_record_offset":41808,"ancestor_record_offset":40512,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":41688,"parent_record_offset":40652,"ancestor_record_offset":40512,"inlinee":21709,"source_line":286,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":40512,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":40684,"parent_record_offset":40652,"ancestor_record_offset":40512,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":40652,"parent_record_offset":40620,"ancestor_record_offset":40512,"inlinee":21707,"source_line":194,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":45212,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":45416,"parent_record_offset":45392,"ancestor_record_offset":45212,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":40588,"parent_record_offset":40560,"ancestor_record_offset":40512,"inlinee":21722,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 676
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11253
// 首条位置: {"module":1239,"inlinee":70227}
// 编译实例 RVAs: 0x2346470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18651208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697136,"raw_hex":"e8b867e3fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5405697187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141191450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18567303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697136,"raw_hex":"e879afe4fe","size":5,"soft_fail":false,"target_va":5387129936,"va":5405697234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411953c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18551037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405697136,"raw_hex":"e803efe4fe","size":5,"soft_fail":false,"target_va":5387146176,"va":5405697208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":1239,"record_offset":249160,"parent_record_offset":249128,"ancestor_record_offset":249128,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 679
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11227
// 首条位置: {"module":1239,"inlinee":70183}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":103156,"parent_record_offset":103120,"ancestor_record_offset":103120,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":103440,"parent_record_offset":103120,"ancestor_record_offset":103120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":103240,"parent_record_offset":103120,"ancestor_record_offset":103120,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":103616,"parent_record_offset":103592,"ancestor_record_offset":103120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":103640,"parent_record_offset":103616,"ancestor_record_offset":103120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":103664,"parent_record_offset":103640,"ancestor_record_offset":103120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":103592,"parent_record_offset":103536,"ancestor_record_offset":103120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":103204,"parent_record_offset":103180,"ancestor_record_offset":103120,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":103564,"parent_record_offset":103536,"ancestor_record_offset":103120,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":103536,"parent_record_offset":103512,"ancestor_record_offset":103120,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":103512,"parent_record_offset":103488,"ancestor_record_offset":103120,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":103180,"parent_record_offset":103156,"ancestor_record_offset":103120,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":103464,"parent_record_offset":103440,"ancestor_record_offset":103120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":103348,"parent_record_offset":103292,"ancestor_record_offset":103120,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":103292,"parent_record_offset":103240,"ancestor_record_offset":103120,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":103264,"parent_record_offset":103240,"ancestor_record_offset":103120,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":103320,"parent_record_offset":103292,"ancestor_record_offset":103120,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":103372,"parent_record_offset":103348,"ancestor_record_offset":103120,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":103404,"parent_record_offset":103292,"ancestor_record_offset":103120,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 683
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":225892,"rva":18478512,"va":5387187632}
// 编译实例 RVAs: 0x119f5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11509296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8d06150ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387187995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13703366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e83ae72eff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387187745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13702988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8b4e82eff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387187767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6123549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8e38fa2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387187720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-164290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e83e7efdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387187837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-162344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8d885fdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387187795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e849410000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387187970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8e8450000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387187811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2392832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8007ddbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387187691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5172833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e89f11b1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387187884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5175692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e87406b1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387187927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10979420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387187632,"raw_hex":"e8a47758ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387187783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":226060,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":226584,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226404,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226344,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226464,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226528,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226140,"parent_record_offset":225892,"ancestor_record_offset":225892,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":226760,"parent_record_offset":226736,"ancestor_record_offset":225892,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":226784,"parent_record_offset":226760,"ancestor_record_offset":225892,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":226808,"parent_record_offset":226784,"ancestor_record_offset":225892,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":226736,"parent_record_offset":226680,"ancestor_record_offset":225892,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":226108,"parent_record_offset":226084,"ancestor_record_offset":225892,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":226708,"parent_record_offset":226680,"ancestor_record_offset":225892,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":226680,"parent_record_offset":226656,"ancestor_record_offset":225892,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":226656,"parent_record_offset":226632,"ancestor_record_offset":225892,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":226084,"parent_record_offset":226060,"ancestor_record_offset":225892,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":226608,"parent_record_offset":226584,"ancestor_record_offset":225892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":226252,"parent_record_offset":226192,"ancestor_record_offset":225892,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":226192,"parent_record_offset":226140,"ancestor_record_offset":225892,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":226164,"parent_record_offset":226140,"ancestor_record_offset":225892,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":226224,"parent_record_offset":226192,"ancestor_record_offset":225892,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":226276,"parent_record_offset":226252,"ancestor_record_offset":225892,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226372,"parent_record_offset":226344,"ancestor_record_offset":225892,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":226308,"parent_record_offset":226192,"ancestor_record_offset":225892,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::load_session_meta
// 记录形式: compiled_procedure; instances=1
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2893,"record_offset":4964,"rva":36987216,"va":5405696336}
// 编译实例 RVAs: 0x2346150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21040170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696336,"raw_hex":"e8d6f3befe","size":5,"soft_fail":false,"target_va":5384656240,"va":5405696405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-30017752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696336,"raw_hex":"e828f735fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405696451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25429061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696336,"raw_hex":"e8bbfb7bfe","size":5,"soft_fail":false,"target_va":5380267312,"va":5405696368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25433551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696336,"raw_hex":"e831ea7bfe","size":5,"soft_fail":false,"target_va":5380262880,"va":5405696426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":5120,"parent_record_offset":4964,"ancestor_record_offset":4964,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2893,"record_offset":5092,"parent_record_offset":4964,"ancestor_record_offset":4964,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":4964,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4964,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":4964,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2893,"record_offset":5316,"parent_record_offset":5264,"ancestor_record_offset":4964,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":4964,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":4964,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":5240,"parent_record_offset":5220,"ancestor_record_offset":4964,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":5140,"parent_record_offset":5120,"ancestor_record_offset":4964,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons
// 记录形式: compiled_procedure; instances=1
// 源码行: 698
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2893,"record_offset":6540,"rva":36987504,"va":5405696624}
// 编译实例 RVAs: 0x2346270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24632615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696624,"raw_hex":"e8d92288fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405696786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18650991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696624,"raw_hex":"e89168e3fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5405696970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141193bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18557009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696624,"raw_hex":"e8afd7e4fe","size":5,"soft_fail":false,"target_va":5387140080,"va":5405697084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10654541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696624,"raw_hex":"e84d93a200","size":5,"soft_fail":false,"target_va":5416351392,"va":5405696846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29490447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405696624,"raw_hex":"e8f1023efe","size":5,"soft_fail":false,"target_va":5376206368,"va":5405696810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2893,"record_offset":6676,"parent_record_offset":6540,"ancestor_record_offset":6540,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2893,"record_offset":7048,"parent_record_offset":6540,"ancestor_record_offset":6540,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":6732,"parent_record_offset":6540,"ancestor_record_offset":6540,"inlinee":21728,"source_line":488,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6540,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6540,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2893,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6540,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":6884,"parent_record_offset":6848,"ancestor_record_offset":6540,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":6824,"parent_record_offset":6756,"ancestor_record_offset":6540,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2893,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":6540,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2893,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":6540,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2893,"record_offset":7004,"parent_record_offset":6848,"ancestor_record_offset":6540,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":6540,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2893,"record_offset":6796,"parent_record_offset":6756,"ancestor_record_offset":6540,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 728
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11248
// 首条位置: {"module":1239,"inlinee":70216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":212044,"parent_record_offset":212016,"ancestor_record_offset":212016,"inlinee":22049,"source_line":330,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":212100,"parent_record_offset":212016,"ancestor_record_offset":212016,"inlinee":22041,"source_line":325,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":212068,"parent_record_offset":212044,"ancestor_record_offset":212016,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 753
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1124A
// 首条位置: {"module":1239,"inlinee":70218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":213300,"parent_record_offset":213272,"ancestor_record_offset":213272,"inlinee":22049,"source_line":330,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":213356,"parent_record_offset":213272,"ancestor_record_offset":213272,"inlinee":22041,"source_line":325,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":213324,"parent_record_offset":213300,"ancestor_record_offset":213272,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 773
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11249
// 首条位置: {"module":1239,"inlinee":70217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":214540,"parent_record_offset":214512,"ancestor_record_offset":214512,"inlinee":22049,"source_line":330,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":214596,"parent_record_offset":214512,"ancestor_record_offset":214512,"inlinee":22041,"source_line":325,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":214564,"parent_record_offset":214540,"ancestor_record_offset":214512,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 792
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1124B
// 首条位置: {"module":1239,"inlinee":70219}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":222636,"parent_record_offset":222612,"ancestor_record_offset":222612,"inlinee":22049,"source_line":330,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":222660,"parent_record_offset":222636,"ancestor_record_offset":222612,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
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
// 完整结构及原始类型记录: load.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::LocalState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::load::LocalState>", "constructor": "bevy_platform::cell::sync_cell::SyncCell", "arguments": ["toolbar::menu::load::LocalState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::load::LocalState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::load::LocalState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::menu::load::LocalState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > > >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$130::init_access<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >", "constructor": "bevy_ecs::system::system_param::impl$130::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::SaveLoadError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::load::SaveLoadError>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::load::SaveLoadError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error
// 信息 ID: c4f6c9fdba246015d8dcf2e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x23474b0 -> 0x2346470 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$9; module=1239, inline_record=249128; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error::closure$0
// 信息 ID: 6f1ca83c79338a4b611fd5e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::display_save_file_load_error", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::display_save_file_load_error::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1239, inline_record=103120; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1239, inline_record=103756; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1239, inline_record=104360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error::closure$1
// 信息 ID: 84c80145eedc5b7af49e9208
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::load::display_save_file_load_error", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::display_save_file_load_error::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$1,ref$<str$> > @ 0x11916d8 -> 0x119f5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$0::default
// 信息 ID: 5b852c0b3b054b52388adcc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9454405fbe361049341b0a44: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)>; module=1296, inline_record=5244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::load_session_meta
// 信息 ID: 73b6258aa778ab01abae0cee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a0157 -> 0x2346150 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x2347dcb -> 0x2346150 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x2346d10 -> 0x2346150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons
// 信息 ID: a64175750150ac8e5b9f6231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x23472ee -> 0x2346270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0
// 信息 ID: 5095ab1619db122d9da8d802
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::show_contextual_buttons::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::show_contextual_buttons::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::show_contextual_buttons::closure_env$0,ref$<str$> > @ 0x1193e78 -> 0x119e560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
// 信息 ID: 6c7c7f6d41cea75b120e69f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1239, inline_record=212016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
// 信息 ID: 88b6056e8b5d542158720df4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1239, inline_record=213272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
// 信息 ID: f20dc115c98874c84bc72c8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1239, inline_record=214512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$3
// 信息 ID: 19115cbfc52e5563c8f2bf72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1239, inline_record=222612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui
// 信息 ID: ac92c6afcfb5d54da0fddc83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9454405fbe361049341b0a44: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> @ 0x11e0cc8 -> 0x23464e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::DeleteSessionCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >)> > @ 0x231101b -> 0x23464e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$0
// 信息 ID: 5fd1c53c171621570aac8aca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2893, inline_record=10320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$0::closure$0
// 信息 ID: 45a7d548ed922d3b2cd00de6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2893, inline_record=10440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$10
// 信息 ID: b5dc6dd6202e2e038821b88a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$10 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$10,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$10,ref$<str$> > @ 0x1195ea8 -> 0x119f770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$10::closure$0
// 信息 ID: 3c6603725a0a8ccda4895a20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure$10::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::load::ui::closure$10::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::load::ui::closure$10::closure_env$0>; module=1235, inline_record=126888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$11
// 信息 ID: 31103afc24c28488de424a03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$11 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$11,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$11,ref$<str$> >; module=1239, inline_record=40512; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$11,ref$<str$> >; module=1239, inline_record=42876; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$11,ref$<str$> >; module=1239, inline_record=45212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$2
// 信息 ID: cc4b57dc0bea01e0389833e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2893, inline_record=12980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$4
// 信息 ID: e43a47c102efac44d9391523
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$4,ref$<str$> >; module=1239, inline_record=74392; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$4,ref$<str$> >; module=1239, inline_record=75820; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$4,ref$<str$> >; module=1239, inline_record=77220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$5
// 信息 ID: 06cd0c5639b4ef7462e0a907
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1239, inline_record=63432; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1239, inline_record=64860; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1239, inline_record=66260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$6
// 信息 ID: 1780cc7eb70abd2aa89fba70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2893, inline_record=23312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$7
// 信息 ID: 7fc301fe48c062859632b108
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$7 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> >; module=1239, inline_record=157872; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> >; module=1239, inline_record=158100; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> >; module=1239, inline_record=158300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$8
// 信息 ID: 0717b46d8c22d3ebbf59a98c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$8 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$8 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$8 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$8 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; module=4247, inline_record=744; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; module=4247, inline_record=912; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; module=4865, inline_record=3744; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >; module=4865, inline_record=3912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$9
// 信息 ID: f38d4e5bfec1394a9d5d46fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$9 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> > @ 0x1198438 -> 0x119fa90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$9::closure$0
// 信息 ID: 2af1ccab89fbd74bb787e3e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$9; module=1239, inline_record=237856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$9::closure$0::closure$0
// 信息 ID: 06c259521e226caa58ae0cee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$9; module=1239, inline_record=237968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
