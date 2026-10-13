// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/mod_row.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 834456cc9069284d0dcc63d879a44552f1c16f72ad11411f84b45582fe8da925
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod_row.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4209
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4209,"record_offset":18312,"rva":43461104,"va":5412170224}
// 编译实例 RVAs: 0x29729f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-36494789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e83b22d3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412173472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-36492580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8dc2ad3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412171279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43451266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e87efc68fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412173453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=7
//     target_aliases=6 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38691481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8679db1fd","size":5,"soft_fail":false,"target_va":5373479040,"va":5412170516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38685929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e817b3b1fd","size":5,"soft_fail":false,"target_va":5373484384,"va":5412170308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38685546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e896b4b1fd","size":5,"soft_fail":false,"target_va":5373484784,"va":5412170325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35935377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e86fabdbfd","size":5,"soft_fail":false,"target_va":5376236864,"va":5412172236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31106644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8ac5925fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412170815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30565914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8e6992dfe","size":5,"soft_fail":false,"target_va":5381607664,"va":5412173573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25151518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8e23780fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5412172249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25147403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8f54780fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5412170950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25145440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8a04f80fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5412170891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25124404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8cca180fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412170383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25050224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e890c381fe","size":5,"soft_fail":false,"target_va":5387120192,"va":5412170411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24966284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8740b83fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5412170967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24965203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8ad0f83fe","size":5,"soft_fail":false,"target_va":5387205712,"va":5412170910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422f5c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6805160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8582998ff","size":5,"soft_fail":false,"target_va":5405367440,"va":5412172595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4883557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e865844a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5412173494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-27377346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e83e415efe","size":5,"soft_fail":false,"target_va":5384794864,"va":5412172205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4447411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e8b3dc4300","size":5,"soft_fail":false,"target_va":5416620794,"va":5412173378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4448117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e875df4300","size":5,"soft_fail":false,"target_va":5416621536,"va":5412173414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4449870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e84ee64300","size":5,"soft_fail":false,"target_va":5416621600,"va":5412171725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::ModEditor::open_mod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32293334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e82a3e13fe","size":5,"soft_fail":false,"target_va":5379880224,"va":5412173553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29037432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e888ec44fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5412172147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30158783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e841d033fe","size":5,"soft_fail":false,"target_va":5382012240,"va":5412171018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35961983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e88143dbfd","size":5,"soft_fail":false,"target_va":5376208368,"va":5412170346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::ellipsize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35919358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412170224,"raw_hex":"e802eadbfd","size":5,"soft_fail":false,"target_va":5376251424,"va":5412170777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":21256,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4209,"record_offset":18648,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":21200,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":18536,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":18788,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":21804,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":20060,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":20000,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":19796,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":20124,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::mods_mine_tab::list_slot::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_mine_tab.rs
//     evidence={"module":4209,"record_offset":24164,"parent_record_offset":18312,"ancestor_record_offset":18312,"inlinee":136692,"source_line":73,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":18312,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":18312,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":21432,"parent_record_offset":21408,"ancestor_record_offset":18312,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":21456,"parent_record_offset":21432,"ancestor_record_offset":18312,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":29124,"parent_record_offset":29100,"ancestor_record_offset":18312,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":21480,"parent_record_offset":21456,"ancestor_record_offset":18312,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":29052,"parent_record_offset":28900,"ancestor_record_offset":18312,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":21408,"parent_record_offset":21352,"ancestor_record_offset":18312,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":24280,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":24328,"parent_record_offset":24304,"ancestor_record_offset":18312,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":28836,"parent_record_offset":28808,"ancestor_record_offset":18312,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":21380,"parent_record_offset":21352,"ancestor_record_offset":18312,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":21352,"parent_record_offset":21328,"ancestor_record_offset":18312,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":28900,"parent_record_offset":28864,"ancestor_record_offset":18312,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":28864,"parent_record_offset":28836,"ancestor_record_offset":18312,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":21328,"parent_record_offset":21304,"ancestor_record_offset":18312,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":28940,"parent_record_offset":28900,"ancestor_record_offset":18312,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4209,"record_offset":28776,"parent_record_offset":28712,"ancestor_record_offset":18312,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":28748,"parent_record_offset":28712,"ancestor_record_offset":18312,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":28808,"parent_record_offset":28776,"ancestor_record_offset":18312,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":28684,"parent_record_offset":28656,"ancestor_record_offset":18312,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":28712,"parent_record_offset":28684,"ancestor_record_offset":18312,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":4209,"record_offset":28656,"parent_record_offset":28628,"ancestor_record_offset":18312,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4209,"record_offset":28964,"parent_record_offset":28940,"ancestor_record_offset":18312,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4209,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":18312,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":18312,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":19204,"parent_record_offset":19180,"ancestor_record_offset":18312,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":19620,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":29012,"parent_record_offset":28988,"ancestor_record_offset":18312,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4209,"record_offset":28988,"parent_record_offset":28964,"ancestor_record_offset":18312,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":19352,"parent_record_offset":19324,"ancestor_record_offset":18312,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":19324,"parent_record_offset":19296,"ancestor_record_offset":18312,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":18312,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":21164,"parent_record_offset":20156,"ancestor_record_offset":18312,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4209,"record_offset":29180,"parent_record_offset":28776,"ancestor_record_offset":18312,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":18312,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":18312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4209,"record_offset":19468,"parent_record_offset":19444,"ancestor_record_offset":18312,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":18820,"parent_record_offset":18788,"ancestor_record_offset":18312,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":24488,"parent_record_offset":24456,"ancestor_record_offset":18312,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4209,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":18312,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":19264,"parent_record_offset":19236,"ancestor_record_offset":18312,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":19296,"parent_record_offset":19264,"ancestor_record_offset":18312,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":19236,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":19096,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":19648,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":19180,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4209,"record_offset":19040,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4209,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":18312,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":24592,"parent_record_offset":24552,"ancestor_record_offset":18312,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":18876,"parent_record_offset":18848,"ancestor_record_offset":18312,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":19504,"parent_record_offset":19396,"ancestor_record_offset":18312,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":18908,"parent_record_offset":18876,"ancestor_record_offset":18312,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":19444,"parent_record_offset":19420,"ancestor_record_offset":18312,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":19740,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":19420,"parent_record_offset":19396,"ancestor_record_offset":18312,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":18312,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":24652,"parent_record_offset":24592,"ancestor_record_offset":18312,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":18936,"parent_record_offset":18876,"ancestor_record_offset":18312,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":18312,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":19068,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":19396,"parent_record_offset":18984,"ancestor_record_offset":18312,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":24552,"parent_record_offset":24516,"ancestor_record_offset":18312,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":18848,"parent_record_offset":18788,"ancestor_record_offset":18312,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":24516,"parent_record_offset":24456,"ancestor_record_offset":18312,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":4209,"record_offset":28600,"parent_record_offset":28572,"ancestor_record_offset":18312,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":4209,"record_offset":28572,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4209,"record_offset":28628,"parent_record_offset":28600,"ancestor_record_offset":18312,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":29264,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":25520,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":19908,"parent_record_offset":19848,"ancestor_record_offset":18312,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":19848,"parent_record_offset":19796,"ancestor_record_offset":18312,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":24456,"parent_record_offset":24424,"ancestor_record_offset":18312,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":20156,"parent_record_offset":20124,"ancestor_record_offset":18312,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":18312,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":25548,"parent_record_offset":25520,"ancestor_record_offset":18312,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":25572,"parent_record_offset":25548,"ancestor_record_offset":18312,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":18312,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":18312,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":18612,"parent_record_offset":18560,"ancestor_record_offset":18312,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":18312,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":27396,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":18312,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":27336,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":24392,"parent_record_offset":24364,"ancestor_record_offset":18312,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":24364,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":27132,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":19964,"parent_record_offset":19848,"ancestor_record_offset":18312,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":27456,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":24424,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_mine_tab::list_slot::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_mine_tab.rs
//     evidence={"module":4209,"record_offset":29320,"parent_record_offset":24164,"ancestor_record_offset":18312,"inlinee":111835,"source_line":48,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4209,"record_offset":264,"rva":43455776,"va":5412164896}
// 编译实例 RVAs: 0x2971520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-36488478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8e23ad3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412167161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-36487252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8ac3fd3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412165951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43444955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8251569fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412167142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38686153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e837b2b1fd","size":5,"soft_fail":false,"target_va":5373479040,"va":5412165188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38680601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8e7c7b1fd","size":5,"soft_fail":false,"target_va":5373484384,"va":5412164980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38680218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e866c9b1fd","size":5,"soft_fail":false,"target_va":5373484784,"va":5412164997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35929961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e897c0dbfd","size":5,"soft_fail":false,"target_va":5376236864,"va":5412166820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31101316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e87c6e25fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412165487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30559602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e88eb22dfe","size":5,"soft_fail":false,"target_va":5381607664,"va":5412167261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25146097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e80f4d80fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5412166828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25142072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8c85c80fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5412165619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25140112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8706480fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5412165563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25119076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e89cb680fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412165055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25044896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e860d881fe","size":5,"soft_fail":false,"target_va":5387120192,"va":5412165083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24960953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8472083fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5412165636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24959875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e87d2483fe","size":5,"soft_fail":false,"target_va":5387205712,"va":5412165582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4889868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e80c9d4a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5412167183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4453694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e83ef54300","size":5,"soft_fail":false,"target_va":5416620794,"va":5412167095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4454400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e800f84300","size":5,"soft_fail":false,"target_va":5416621536,"va":5412167131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4455198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e81efb4300","size":5,"soft_fail":false,"target_va":5416621600,"va":5412166397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::mod_editor::ModEditor::open_mod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32287022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8d25613fe","size":5,"soft_fail":false,"target_va":5379880224,"va":5412167241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29032080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8700145fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5412166795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30153452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e814e533fe","size":5,"soft_fail":false,"target_va":5382012240,"va":5412165687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35956655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e85158dbfd","size":5,"soft_fail":false,"target_va":5376208368,"va":5412165018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::ellipsize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35914030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412164896,"raw_hex":"e8d2fedbfd","size":5,"soft_fail":false,"target_va":5376251424,"va":5412165449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":3208,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4209,"record_offset":600,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":3152,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":488,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":740,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":3756,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":2012,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":1952,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":1748,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":2076,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::mods_mine_tab::list_slot::closure$1::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_mine_tab.rs
//     evidence={"module":4209,"record_offset":6160,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":136690,"source_line":110,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":7908,"parent_record_offset":7756,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":3360,"parent_record_offset":3304,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":7756,"parent_record_offset":7720,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":7720,"parent_record_offset":7692,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":7796,"parent_record_offset":7756,"ancestor_record_offset":264,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4209,"record_offset":7632,"parent_record_offset":7568,"ancestor_record_offset":264,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":7604,"parent_record_offset":7568,"ancestor_record_offset":264,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":7664,"parent_record_offset":7632,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":264,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":7568,"parent_record_offset":7540,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":4209,"record_offset":7512,"parent_record_offset":7484,"ancestor_record_offset":264,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4209,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":264,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4209,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":264,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":264,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":1572,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":264,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4209,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":264,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":264,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":264,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":3116,"parent_record_offset":2108,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4209,"record_offset":8036,"parent_record_offset":7632,"ancestor_record_offset":264,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4209,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":264,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":264,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":6316,"parent_record_offset":6288,"ancestor_record_offset":264,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4209,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":264,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":264,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":264,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":1188,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":1048,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":1600,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":1132,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4209,"record_offset":992,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4209,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":264,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":6416,"parent_record_offset":6376,"ancestor_record_offset":264,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":264,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":1456,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":860,"parent_record_offset":828,"ancestor_record_offset":264,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":264,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":1692,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":264,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":6476,"parent_record_offset":6416,"ancestor_record_offset":264,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":888,"parent_record_offset":828,"ancestor_record_offset":264,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":264,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":1020,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":1348,"parent_record_offset":936,"ancestor_record_offset":264,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":6376,"parent_record_offset":6344,"ancestor_record_offset":264,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":800,"parent_record_offset":740,"ancestor_record_offset":264,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":6344,"parent_record_offset":6288,"ancestor_record_offset":264,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":4209,"record_offset":7456,"parent_record_offset":7428,"ancestor_record_offset":264,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":4209,"record_offset":7428,"parent_record_offset":6160,"ancestor_record_offset":264,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4209,"record_offset":7484,"parent_record_offset":7456,"ancestor_record_offset":264,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":7344,"parent_record_offset":6160,"ancestor_record_offset":264,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":1860,"parent_record_offset":1800,"ancestor_record_offset":264,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":1800,"parent_record_offset":1748,"ancestor_record_offset":264,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":6288,"parent_record_offset":6260,"ancestor_record_offset":264,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":2108,"parent_record_offset":2076,"ancestor_record_offset":264,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":264,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":264,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":264,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":264,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":564,"parent_record_offset":512,"ancestor_record_offset":264,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":264,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":264,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":6228,"parent_record_offset":6200,"ancestor_record_offset":264,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":6200,"parent_record_offset":6160,"ancestor_record_offset":264,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":1916,"parent_record_offset":1800,"ancestor_record_offset":264,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":6260,"parent_record_offset":6160,"ancestor_record_offset":264,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::mods_mine_tab::list_slot::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_mine_tab.rs
//     evidence={"module":4209,"record_offset":8120,"parent_record_offset":6160,"ancestor_record_offset":264,"inlinee":111835,"source_line":48,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4209,"record_offset":8444,"rva":43458288,"va":5412167408}
// 编译实例 RVAs: 0x2971ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-36489694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e82236d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412168393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=6
//     target_aliases=6 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38688654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e872a8b1fd","size":5,"soft_fail":false,"target_va":5373479040,"va":5412167689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38683105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e81fbeb1fd","size":5,"soft_fail":false,"target_va":5373484384,"va":5412167484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38682722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e89ebfb1fd","size":5,"soft_fail":false,"target_va":5373484784,"va":5412167501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35932396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e814b7dbfd","size":5,"soft_fail":false,"target_va":5376236864,"va":5412169255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31103800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8c86425fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412167971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25148534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e88a4380fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5412169265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25144546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e81e5380fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5412168093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25142596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8bc5a80fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5412168047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25121577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8d7ac80fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412167556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25047397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e89bce81fe","size":5,"soft_fail":false,"target_va":5387120192,"va":5412167584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24963427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e89d1683fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5412168110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24962356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8cc1a83fe","size":5,"soft_fail":false,"target_va":5387205712,"va":5412168063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422f5c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6802143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8213598ff","size":5,"soft_fail":false,"target_va":5405367440,"va":5412169578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_row.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4181146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e89acc3f00","size":5,"soft_fail":false,"target_va":5416351056,"va":5412169905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-27374371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8dd4c5efe","size":5,"soft_fail":false,"target_va":5384794864,"va":5412169230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4451542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8d6ec4300","size":5,"soft_fail":false,"target_va":5416621536,"va":5412169989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4452796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e8bcf14300","size":5,"soft_fail":false,"target_va":5416621600,"va":5412168799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29034472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e818f844fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5412169187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30155920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e870db33fe","size":5,"soft_fail":false,"target_va":5382012240,"va":5412168155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35959156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e88c4edbfd","size":5,"soft_fail":false,"target_va":5376208368,"va":5412167519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::ellipsize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35916523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412167408,"raw_hex":"e815f5dbfd","size":5,"soft_fail":false,"target_va":5376251424,"va":5412167942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":11392,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4209,"record_offset":8784,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":11336,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":8672,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":8924,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":11940,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":10196,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":10136,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":9932,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":10260,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mods_workshop_tab.rs
//     evidence={"module":4209,"record_offset":14344,"parent_record_offset":8444,"ancestor_record_offset":8444,"inlinee":136691,"source_line":171,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":8444,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":8444,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":8444,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4209,"record_offset":11544,"parent_record_offset":11488,"ancestor_record_offset":8444,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":14436,"parent_record_offset":14344,"ancestor_record_offset":8444,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4209,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":8444,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17600,"parent_record_offset":17568,"ancestor_record_offset":8444,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":11516,"parent_record_offset":11488,"ancestor_record_offset":8444,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":8444,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17648,"parent_record_offset":17624,"ancestor_record_offset":8444,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":17624,"parent_record_offset":17600,"ancestor_record_offset":8444,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4209,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":8444,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17568,"parent_record_offset":17528,"ancestor_record_offset":8444,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17496,"parent_record_offset":17460,"ancestor_record_offset":8444,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4209,"record_offset":17528,"parent_record_offset":17496,"ancestor_record_offset":8444,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4209,"record_offset":17432,"parent_record_offset":17396,"ancestor_record_offset":8444,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4209,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":8444,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4209,"record_offset":17372,"parent_record_offset":14344,"ancestor_record_offset":8444,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4209,"record_offset":17460,"parent_record_offset":17396,"ancestor_record_offset":8444,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4209,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":8444,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":8444,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4209,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":8444,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4209,"record_offset":9756,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":9488,"parent_record_offset":9460,"ancestor_record_offset":8444,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4209,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":8444,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":14460,"parent_record_offset":14436,"ancestor_record_offset":8444,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4209,"record_offset":11300,"parent_record_offset":10292,"ancestor_record_offset":8444,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":8444,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":8444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4209,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":8444,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":8956,"parent_record_offset":8924,"ancestor_record_offset":8444,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4209,"record_offset":14644,"parent_record_offset":14612,"ancestor_record_offset":8444,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4209,"record_offset":17688,"parent_record_offset":17528,"ancestor_record_offset":8444,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4209,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":8444,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":9400,"parent_record_offset":9372,"ancestor_record_offset":8444,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":9432,"parent_record_offset":9400,"ancestor_record_offset":8444,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4209,"record_offset":9372,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":9232,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":9784,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4209,"record_offset":9316,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4209,"record_offset":9176,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4209,"record_offset":9664,"parent_record_offset":9640,"ancestor_record_offset":8444,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":14748,"parent_record_offset":14708,"ancestor_record_offset":8444,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":8444,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":9640,"parent_record_offset":9532,"ancestor_record_offset":8444,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4209,"record_offset":9044,"parent_record_offset":9012,"ancestor_record_offset":8444,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9580,"parent_record_offset":9556,"ancestor_record_offset":8444,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9876,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":8444,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":8444,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":14808,"parent_record_offset":14748,"ancestor_record_offset":8444,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9072,"parent_record_offset":9012,"ancestor_record_offset":8444,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":8444,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9204,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4209,"record_offset":9532,"parent_record_offset":9120,"ancestor_record_offset":8444,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":14708,"parent_record_offset":14672,"ancestor_record_offset":8444,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":8984,"parent_record_offset":8924,"ancestor_record_offset":8444,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4209,"record_offset":14672,"parent_record_offset":14612,"ancestor_record_offset":8444,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":15676,"parent_record_offset":14344,"ancestor_record_offset":8444,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":10044,"parent_record_offset":9984,"ancestor_record_offset":8444,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":9984,"parent_record_offset":9932,"ancestor_record_offset":8444,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":14612,"parent_record_offset":14580,"ancestor_record_offset":8444,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":10292,"parent_record_offset":10260,"ancestor_record_offset":8444,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":8444,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":15704,"parent_record_offset":15676,"ancestor_record_offset":8444,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4209,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":8444,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":8444,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":8444,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":8748,"parent_record_offset":8696,"ancestor_record_offset":8444,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4209,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8444,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":10164,"parent_record_offset":10136,"ancestor_record_offset":8444,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":8444,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14520,"parent_record_offset":14344,"ancestor_record_offset":8444,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":10100,"parent_record_offset":9984,"ancestor_record_offset":8444,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/04209.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4209,"record_offset":14580,"parent_record_offset":14344,"ancestor_record_offset":8444,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/04209.jsonl.gz"}
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
// 完整结构及原始类型记录: mod_row.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod_row.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod_row.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$1>
// 信息 ID: 28b08fdf6d89ab8270df90fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::mod_row::mod_row -> ["toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_mine_tab::list_slot::closure_env$1> @ 0x231d3b9 -> 0x29729f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$3>
// 信息 ID: b3267e2b8933f3e411728e59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::mod_row::mod_row -> ["toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_mine_tab::list_slot::closure_env$1> @ 0x231d47e -> 0x2971520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mod_row::mod_row<toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2>
// 信息 ID: 5c32a6a561103ad2964a8f1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::mod_row::mod_row -> ["toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::mods_workshop_tab::list_slot::closure_env$3> @ 0x231c823 -> 0x2971ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
