// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-mods/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f1094c5c81a0939a02ef6dc936f94d33a3da52e89f981ed5c62db79c0c766b66
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 680, 681, 683, 684, 1778, 1793, 2025, 2026, 2034, 2043, 2049, 3567, 3568
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: alloc::collections::btree::node::Handle::insert_recursing<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2034,"record_offset":7700,"rva":31097200,"va":5399806320}
// 编译实例 RVAs: 0x1da8170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24128190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e842d58ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5399806873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-24131715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e87dc78ffe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399810414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31084671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e881af25fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5399806858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3150
//     owner=unresolved_namespace; count=6
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e8fb9fffff","size":5,"soft_fail":false,"target_va":5399785808,"va":5399810384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17249961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e8a9360701","size":5,"soft_fail":false,"target_va":5417057056,"va":5399807090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17250522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e8da380701","size":5,"soft_fail":false,"target_va":5417057072,"va":5399806545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16810652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e89c820001","size":5,"soft_fail":false,"target_va":5416620769,"va":5399810112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16811346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e852850001","size":5,"soft_fail":false,"target_va":5416621600,"va":5399810249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16812170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e88a880001","size":5,"soft_fail":false,"target_va":5416622208,"va":5399810033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tuple$<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e88becffff","size":5,"soft_fail":false,"target_va":5399806128,"va":5399811104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16812253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399806320,"raw_hex":"e8dd880001","size":5,"soft_fail":false,"target_va":5416622320,"va":5399810062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::map::entry::impl$8::insert_entry::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map\entry.rs
//     evidence={"module":2034,"record_offset":16064,"parent_record_offset":7700,"ancestor_record_offset":7700,"inlinee":11650,"source_line":411,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::Handle::awaken
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":10984,"parent_record_offset":7700,"ancestor_record_offset":7700,"inlinee":11644,"source_line":867,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::Handle::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8108,"parent_record_offset":7700,"ancestor_record_offset":7700,"inlinee":11632,"source_line":1020,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":11012,"parent_record_offset":7700,"ancestor_record_offset":7700,"inlinee":4917,"source_line":328,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":8768,"parent_record_offset":8744,"ancestor_record_offset":7700,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":7700,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":7700,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":7700,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":7700,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":7700,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":7700,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2034,"record_offset":10008,"parent_record_offset":9952,"ancestor_record_offset":7700,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2034,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":7700,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2034,"record_offset":8688,"parent_record_offset":8652,"ancestor_record_offset":7700,"inlinee":10871,"source_line":574,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2034,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":7700,"inlinee":10872,"source_line":606,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2034,"record_offset":16232,"parent_record_offset":16196,"ancestor_record_offset":7700,"inlinee":4906,"source_line":18,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2034,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":7700,"inlinee":4926,"source_line":21,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::take_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2034,"record_offset":16196,"parent_record_offset":16160,"ancestor_record_offset":7700,"inlinee":11597,"source_line":9,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::take_mut::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2034,"record_offset":16308,"parent_record_offset":16232,"ancestor_record_offset":7700,"inlinee":11599,"source_line":10,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::correct_parent_link
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":11856,"parent_record_offset":11708,"ancestor_record_offset":7700,"inlinee":11608,"source_line":989,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":11904,"parent_record_offset":11856,"ancestor_record_offset":7700,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::insert_fit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8248,"parent_record_offset":8108,"ancestor_record_offset":7700,"inlinee":11633,"source_line":1002,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::split
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8600,"parent_record_offset":8108,"ancestor_record_offset":7700,"inlinee":11637,"source_line":1288,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::split_leaf_data
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8956,"parent_record_offset":8600,"ancestor_record_offset":7700,"inlinee":11638,"source_line":1221,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::InternalNode::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":12324,"parent_record_offset":12236,"ancestor_record_offset":7700,"inlinee":11604,"source_line":120,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::LeafNode::init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8868,"parent_record_offset":8652,"ancestor_record_offset":7700,"inlinee":11587,"source_line":75,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::LeafNode::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8652,"parent_record_offset":8600,"ancestor_record_offset":7700,"inlinee":11586,"source_line":86,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::as_leaf_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7700,"inlinee":5310,"source_line":310,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::correct_all_childrens_parent_links
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":7700,"inlinee":11606,"source_line":564,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::correct_childrens_parent_links
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":11708,"parent_record_offset":11264,"ancestor_record_offset":7700,"inlinee":11607,"source_line":557,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::edge_area_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":14032,"parent_record_offset":12236,"ancestor_record_offset":7700,"inlinee":11645,"source_line":517,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::from_new_internal
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":14224,"parent_record_offset":12236,"ancestor_record_offset":7700,"inlinee":11605,"source_line":244,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::key_area_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":9064,"parent_record_offset":8956,"ancestor_record_offset":7700,"inlinee":11614,"source_line":487,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8192,"parent_record_offset":8108,"ancestor_record_offset":7700,"inlinee":4914,"source_line":287,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::new_internal
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":16372,"parent_record_offset":16340,"ancestor_record_offset":7700,"inlinee":11602,"source_line":237,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":17296,"parent_record_offset":16064,"ancestor_record_offset":7700,"inlinee":11613,"source_line":693,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::push_internal_level
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":16160,"parent_record_offset":16064,"ancestor_record_offset":7700,"inlinee":11596,"source_line":598,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::set_parent_link
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":12080,"parent_record_offset":11856,"ancestor_record_offset":7700,"inlinee":11609,"source_line":573,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::val_area_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":9212,"parent_record_offset":8956,"ancestor_record_offset":7700,"inlinee":11615,"source_line":501,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$16::ascend::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":7700,"inlinee":4919,"source_line":340,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$30::push_internal_level::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":16340,"parent_record_offset":16308,"ancestor_record_offset":7700,"inlinee":11601,"source_line":602,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::move_to_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":9632,"parent_record_offset":8956,"ancestor_record_offset":7700,"inlinee":11641,"source_line":1875,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::slice_insert
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8300,"parent_record_offset":8248,"ancestor_record_offset":7700,"inlinee":11635,"source_line":1822,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::splitpoint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":2034,"record_offset":8560,"parent_record_offset":8108,"ancestor_record_offset":7700,"inlinee":11636,"source_line":914,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2034,"record_offset":9980,"parent_record_offset":9952,"ancestor_record_offset":7700,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2034,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":7700,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2034,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":7700,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$16::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":2034,"record_offset":9360,"parent_record_offset":8956,"ancestor_record_offset":7700,"inlinee":11113,"source_line":401,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2034,"record_offset":11816,"parent_record_offset":11784,"ancestor_record_offset":7700,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$96::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":2034,"record_offset":10400,"parent_record_offset":10376,"ancestor_record_offset":7700,"inlinee":11643,"source_line":79,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2034,"record_offset":14332,"parent_record_offset":14304,"ancestor_record_offset":7700,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2034,"record_offset":14304,"parent_record_offset":14272,"ancestor_record_offset":7700,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2034,"record_offset":11784,"parent_record_offset":11752,"ancestor_record_offset":7700,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2034,"record_offset":11752,"parent_record_offset":11708,"ancestor_record_offset":7700,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2034,"record_offset":11944,"parent_record_offset":11904,"ancestor_record_offset":7700,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2034,"record_offset":8372,"parent_record_offset":8300,"ancestor_record_offset":7700,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2034,"record_offset":9476,"parent_record_offset":9444,"ancestor_record_offset":7700,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2034,"record_offset":14400,"parent_record_offset":14332,"ancestor_record_offset":7700,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2034,"record_offset":16116,"parent_record_offset":16064,"ancestor_record_offset":7700,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2034,"record_offset":11052,"parent_record_offset":11012,"ancestor_record_offset":7700,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2034,"record_offset":11084,"parent_record_offset":11012,"ancestor_record_offset":7700,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2034,"record_offset":16812,"parent_record_offset":16372,"ancestor_record_offset":7700,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2034,"record_offset":9152,"parent_record_offset":9064,"ancestor_record_offset":7700,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2034,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":7700,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2034,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":7700,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2034,"record_offset":9856,"parent_record_offset":8956,"ancestor_record_offset":7700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2034,"record_offset":8344,"parent_record_offset":8300,"ancestor_record_offset":7700,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2034,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":7700,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2034,"record_offset":9176,"parent_record_offset":9152,"ancestor_record_offset":7700,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2034,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":7700,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2034,"record_offset":9096,"parent_record_offset":9064,"ancestor_record_offset":7700,"inlinee":4871,"source_line":684,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":7700,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9388,"parent_record_offset":9360,"ancestor_record_offset":7700,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":7700,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9568,"parent_record_offset":9544,"ancestor_record_offset":7700,"inlinee":11640,"source_line":417,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":7700,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2034,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":7700,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/02034.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":684,"record_offset":5112,"rva":14246416,"va":5382955536}
// 编译实例 RVAs: 0xd96210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::alloc_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1828368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955536,"raw_hex":"e810e61b00","size":5,"soft_fail":false,"target_va":5384784192,"va":5382955819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::capacity_overflow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1828392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955536,"raw_hex":"e828e61b00","size":5,"soft_fail":false,"target_va":5384784144,"va":5382955747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7277110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955536,"raw_hex":"e8caf590ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382955793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14233593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955536,"raw_hex":"e807d026ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382955780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34101211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955536,"raw_hex":"e8db570802","size":5,"soft_fail":false,"target_va":5417057088,"va":5382955872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":684,"record_offset":5948,"parent_record_offset":5112,"ancestor_record_offset":5112,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::control::tag::TagSliceExt::fill_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":684,"record_offset":5976,"parent_record_offset":5112,"ancestor_record_offset":5112,"inlinee":12598,"source_line":73,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::RawTableInner::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":684,"record_offset":5480,"parent_record_offset":5112,"ancestor_record_offset":5112,"inlinee":12563,"source_line":1596,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::capacity_to_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":684,"record_offset":5284,"parent_record_offset":5112,"ancestor_record_offset":5112,"inlinee":12594,"source_line":105,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":684,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5112,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":684,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":5112,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":684,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":5112,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":684,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5112,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":684,"record_offset":5380,"parent_record_offset":5320,"ancestor_record_offset":5112,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":5680,"parent_record_offset":5532,"ancestor_record_offset":5112,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":5320,"parent_record_offset":5284,"ancestor_record_offset":5112,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":5412,"parent_record_offset":5284,"ancestor_record_offset":5112,"inlinee":12595,"source_line":3806,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":5112,"inlinee":12596,"source_line":3775,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":5112,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":5888,"parent_record_offset":5480,"ancestor_record_offset":5112,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5112,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5112,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":684,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":5112,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":684,"record_offset":5532,"parent_record_offset":5480,"ancestor_record_offset":5112,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::alloc::inner::do_alloc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\alloc.rs
//     evidence={"module":684,"record_offset":5740,"parent_record_offset":5480,"ancestor_record_offset":5112,"inlinee":12568,"source_line":18,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":684,"record_offset":5916,"parent_record_offset":5480,"ancestor_record_offset":5112,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":4688,"rva":1431696,"va":5370140816}
// 编译实例 RVAs: 0x15d890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46183525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e865b4c002","size":5,"soft_fail":false,"target_va":5416324848,"va":5370141318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9f040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46405336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e8d816c402","size":5,"soft_fail":false,"target_va":5416546368,"va":5370141027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":5276,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7635,"source_line":1894,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":5304,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7636,"source_line":1936,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":4956,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5332,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7627,"source_line":639,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5060,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":4868,"parent_record_offset":4688,"ancestor_record_offset":4688,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":680,"record_offset":5496,"parent_record_offset":5332,"ancestor_record_offset":4688,"inlinee":7631,"source_line":625,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":5400,"parent_record_offset":5332,"ancestor_record_offset":4688,"inlinee":7629,"source_line":2397,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":680,"record_offset":5456,"parent_record_offset":5400,"ancestor_record_offset":4688,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":4688,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":4976,"parent_record_offset":4956,"ancestor_record_offset":4688,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":4688,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4688,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":5532,"rva":1432272,"va":5370141392}
// 编译实例 RVAs: 0x15dad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46182949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370141392,"raw_hex":"e825b2c002","size":5,"soft_fail":false,"target_va":5416324848,"va":5370141894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer$LT$R$GT$::parse_exponent_overflow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t46418024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370141392,"raw_hex":"e86848c402","size":5,"soft_fail":false,"target_va":5416559632,"va":5370141603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":6284,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7635,"source_line":1894,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":6312,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7636,"source_line":1936,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5824,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":6340,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7627,"source_line":639,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5980,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5708,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":680,"record_offset":6504,"parent_record_offset":6340,"ancestor_record_offset":5532,"inlinee":7631,"source_line":625,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":6408,"parent_record_offset":6340,"ancestor_record_offset":5532,"inlinee":7629,"source_line":2397,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":680,"record_offset":6464,"parent_record_offset":6408,"ancestor_record_offset":5532,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":5532,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":5844,"parent_record_offset":5824,"ancestor_record_offset":5532,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":5532,"inlinee":7607,"source_line":683,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5532,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":5864,"parent_record_offset":5844,"ancestor_record_offset":5532,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":5532,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5532,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":3340,"rva":14163936,"va":5382873056}
// 编译实例 RVAs: 0xd81fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33449980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e8fc67fe01","size":5,"soft_fail":false,"target_va":5416323136,"va":5382873151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33451354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e85a6dfe01","size":5,"soft_fail":false,"target_va":5416324848,"va":5382873489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9ed40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33672319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fcc0102","size":5,"soft_fail":false,"target_va":5416545600,"va":5382873276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-1018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e806fcffff","size":5,"soft_fail":false,"target_va":5382872496,"va":5382873509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-12732545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fb73dff","size":5,"soft_fail":false,"target_va":5370140816,"va":5382873356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3752,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3508,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_number
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3808,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":7872,"source_line":509,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3564,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3340,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":3868,"parent_record_offset":3808,"ancestor_record_offset":3340,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3340,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":3528,"parent_record_offset":3508,"ancestor_record_offset":3340,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":3620,"parent_record_offset":3588,"ancestor_record_offset":3340,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_number<serde_json::read::StrRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":1896,"rva":1428672,"va":5370137792}
// 编译实例 RVAs: 0x15ccc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46184637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137792,"raw_hex":"e8bdb8c002","size":5,"soft_fail":false,"target_va":5416323136,"va":5370138494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46186268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137792,"raw_hex":"e81cbfc002","size":5,"soft_fail":false,"target_va":5416324848,"va":5370138575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer$LT$R$GT$::parse_decimal_overflow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t46421119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137792,"raw_hex":"e87f54c402","size":5,"soft_fail":false,"target_va":5416559264,"va":5370138140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t3524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137792,"raw_hex":"e8c40d0000","size":5,"soft_fail":false,"target_va":5370141392,"va":5370137863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_decimal
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2156,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":7626,"source_line":530,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2056,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":680,"record_offset":2712,"parent_record_offset":2548,"ancestor_record_offset":1896,"inlinee":7631,"source_line":625,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":680,"record_offset":2616,"parent_record_offset":2548,"ancestor_record_offset":1896,"inlinee":7629,"source_line":2397,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":680,"record_offset":2672,"parent_record_offset":2616,"ancestor_record_offset":1896,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":1896,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::f64_from_parts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2548,"parent_record_offset":2156,"ancestor_record_offset":1896,"inlinee":7627,"source_line":639,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2076,"parent_record_offset":2056,"ancestor_record_offset":1896,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":2332,"parent_record_offset":2156,"ancestor_record_offset":1896,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1896,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":2096,"parent_record_offset":2076,"ancestor_record_offset":1896,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1896,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":2116,"parent_record_offset":2096,"ancestor_record_offset":1896,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_str<serde_json::read::SliceRead,serde_json::raw::BoxedFromString>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":34860,"rva":14173120,"va":5382882240}
// 编译实例 RVAs: 0xd843c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..SliceRead$u20$as$u20$serde_json..read..Read$GT$::parse_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t372391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e8a7ae0500","size":5,"soft_fail":false,"target_va":5383254800,"va":5382882404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7203796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e82c1492ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382882479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14160273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e86fee27ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382882460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33440778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e80a44fe01","size":5,"soft_fail":false,"target_va":5416323136,"va":5382882353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33471568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e850bcfe01","size":5,"soft_fail":false,"target_va":5416354128,"va":5382882555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9e6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33661355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e8aba10102","size":5,"soft_fail":false,"target_va":5416543904,"va":5382882544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34174550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e856760902","size":5,"soft_fail":false,"target_va":5417057056,"va":5382882501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33738336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382882240,"raw_hex":"e860ce0202","size":5,"soft_fail":false,"target_va":5416620794,"va":5382882453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":680,"record_offset":35288,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":35232,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":35904,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":35064,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::raw::impl$15::visit_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/raw.rs
//     evidence={"module":680,"record_offset":35316,"parent_record_offset":34860,"ancestor_record_offset":34860,"inlinee":7772,"source_line":496,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":35720,"parent_record_offset":35696,"ancestor_record_offset":34860,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":35744,"parent_record_offset":35720,"ancestor_record_offset":34860,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":35768,"parent_record_offset":35744,"ancestor_record_offset":34860,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":35696,"parent_record_offset":35544,"ancestor_record_offset":34860,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":35488,"parent_record_offset":35464,"ancestor_record_offset":34860,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":35544,"parent_record_offset":35512,"ancestor_record_offset":34860,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":35512,"parent_record_offset":35488,"ancestor_record_offset":34860,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":35584,"parent_record_offset":35544,"ancestor_record_offset":34860,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":35388,"parent_record_offset":35364,"ancestor_record_offset":34860,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":35412,"parent_record_offset":35388,"ancestor_record_offset":34860,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":35436,"parent_record_offset":35412,"ancestor_record_offset":34860,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":35364,"parent_record_offset":35340,"ancestor_record_offset":34860,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":680,"record_offset":35340,"parent_record_offset":35316,"ancestor_record_offset":34860,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":680,"record_offset":35464,"parent_record_offset":35436,"ancestor_record_offset":34860,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":680,"record_offset":35608,"parent_record_offset":35584,"ancestor_record_offset":34860,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":680,"record_offset":35656,"parent_record_offset":35632,"ancestor_record_offset":34860,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":680,"record_offset":35632,"parent_record_offset":35608,"ancestor_record_offset":34860,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":680,"record_offset":35824,"parent_record_offset":35436,"ancestor_record_offset":34860,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":680,"record_offset":35848,"parent_record_offset":35824,"ancestor_record_offset":34860,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":35172,"parent_record_offset":35064,"ancestor_record_offset":34860,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":35104,"parent_record_offset":35064,"ancestor_record_offset":34860,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":35196,"parent_record_offset":35172,"ancestor_record_offset":34860,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":35132,"parent_record_offset":35104,"ancestor_record_offset":34860,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_str<serde_json::read::StrRead,serde_json::raw::BoxedFromString>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":680,"record_offset":33700,"rva":14172784,"va":5382881904}
// 编译实例 RVAs: 0xd84270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..StrRead$u20$as$u20$serde_json..read..Read$GT$::parse_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t371655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e8c7ab0500","size":5,"soft_fail":false,"target_va":5383253728,"va":5382882068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7203460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e87c1592ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382882143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14159937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e8bfef27ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382882124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33441114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e85a45fe01","size":5,"soft_fail":false,"target_va":5416323136,"va":5382882017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33471904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e8a0bdfe01","size":5,"soft_fail":false,"target_va":5416354128,"va":5382882219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9e9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33662539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e84ba60102","size":5,"soft_fail":false,"target_va":5416544752,"va":5382882208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34174886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e8a6770902","size":5,"soft_fail":false,"target_va":5417057056,"va":5382882165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33738672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382881904,"raw_hex":"e8b0cf0202","size":5,"soft_fail":false,"target_va":5416620794,"va":5382882117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":680,"record_offset":34212,"parent_record_offset":33700,"ancestor_record_offset":33700,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":34128,"parent_record_offset":33700,"ancestor_record_offset":33700,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":34828,"parent_record_offset":33700,"ancestor_record_offset":33700,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":33900,"parent_record_offset":33700,"ancestor_record_offset":33700,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_direct] serde_json::raw::impl$15::visit_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/raw.rs
//     evidence={"module":680,"record_offset":34240,"parent_record_offset":33700,"ancestor_record_offset":33700,"inlinee":7772,"source_line":496,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":34644,"parent_record_offset":34620,"ancestor_record_offset":33700,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":34668,"parent_record_offset":34644,"ancestor_record_offset":33700,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":34692,"parent_record_offset":34668,"ancestor_record_offset":33700,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":680,"record_offset":34620,"parent_record_offset":34468,"ancestor_record_offset":33700,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":34412,"parent_record_offset":34388,"ancestor_record_offset":33700,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":34468,"parent_record_offset":34436,"ancestor_record_offset":33700,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":34436,"parent_record_offset":34412,"ancestor_record_offset":33700,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":680,"record_offset":34508,"parent_record_offset":34468,"ancestor_record_offset":33700,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":34312,"parent_record_offset":34288,"ancestor_record_offset":33700,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":34336,"parent_record_offset":34312,"ancestor_record_offset":33700,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":34360,"parent_record_offset":34336,"ancestor_record_offset":33700,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":680,"record_offset":34288,"parent_record_offset":34264,"ancestor_record_offset":33700,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":680,"record_offset":34264,"parent_record_offset":34240,"ancestor_record_offset":33700,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":680,"record_offset":34388,"parent_record_offset":34360,"ancestor_record_offset":33700,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":680,"record_offset":34532,"parent_record_offset":34508,"ancestor_record_offset":33700,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":680,"record_offset":34580,"parent_record_offset":34556,"ancestor_record_offset":33700,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":680,"record_offset":34556,"parent_record_offset":34532,"ancestor_record_offset":33700,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":680,"record_offset":34748,"parent_record_offset":34360,"ancestor_record_offset":33700,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":680,"record_offset":34772,"parent_record_offset":34748,"ancestor_record_offset":33700,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":34040,"parent_record_offset":33900,"ancestor_record_offset":33700,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":680,"record_offset":33940,"parent_record_offset":33900,"ancestor_record_offset":33700,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":34064,"parent_record_offset":34040,"ancestor_record_offset":33700,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":33968,"parent_record_offset":33940,"ancestor_record_offset":33700,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":34088,"parent_record_offset":34064,"ancestor_record_offset":33700,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":680,"record_offset":33996,"parent_record_offset":33968,"ancestor_record_offset":33700,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::SliceRead> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2026,"record_offset":5920,"rva":31073680,"va":5399782800}
// 编译实例 RVAs: 0x1da2590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17065530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e83a660401","size":5,"soft_fail":false,"target_va":5416848880,"va":5399783345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-24104724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8ec3090fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399783423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29629214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8e2e43bfe","size":5,"soft_fail":false,"target_va":5370153648,"va":5399782857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29629189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8fbe43bfe","size":5,"soft_fail":false,"target_va":5370154480,"va":5399783664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030e750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27868846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e852c156fe","size":5,"soft_fail":false,"target_va":5371914064,"va":5399782905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d82260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16909634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8befafdfe","size":5,"soft_fail":false,"target_va":5382873696,"va":5399783325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d830d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16905996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8f408fefe","size":5,"soft_fail":false,"target_va":5382877392,"va":5399783383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d843c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16901044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e84c1cfefe","size":5,"soft_fail":false,"target_va":5382882240,"va":5399783279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da31f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e831090000","size":5,"soft_fail":false,"target_va":5399785968,"va":5399783610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8e5080000","size":5,"soft_fail":false,"target_va":5399786496,"va":5399784214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=5
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16540092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8bc61fc00","size":5,"soft_fail":false,"target_va":5416323136,"va":5399783039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::hab7539d1bf208f16 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e8adeeffff","size":5,"soft_fail":false,"target_va":5399779808,"va":5399784238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$alloc..string..String$C$serde_json..value..Value$C$alloc..alloc..Global$GT$$GT$::h065e14f06e7b32d2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e88bd9ffff","size":5,"soft_fail":false,"target_va":5399775408,"va":5399785248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$serde_json..value..Value$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e86cebffff","size":5,"soft_fail":false,"target_va":5399779152,"va":5399784415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e80dedffff","size":5,"soft_fail":false,"target_va":5399779584,"va":5399784430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h64cf2b524f0eadd2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399782800,"raw_hex":"e831e7ffff","size":5,"soft_fail":false,"target_va":5399777712,"va":5399784058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":7040,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2026,"record_offset":6976,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":8152,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":7769,"source_line":1927,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":6584,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6152,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6208,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":2026,"record_offset":7776,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":11167,"source_line":127,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":2026,"record_offset":6556,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":11166,"source_line":41,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":5920,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":5920,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":5920,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":7192,"parent_record_offset":7136,"ancestor_record_offset":5920,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2026,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":5920,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":2026,"record_offset":7388,"parent_record_offset":7356,"ancestor_record_offset":5920,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":2026,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":5920,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":5920,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":7164,"parent_record_offset":7136,"ancestor_record_offset":5920,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":5920,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":5920,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":5920,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":5920,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2026,"record_offset":9700,"parent_record_offset":9672,"ancestor_record_offset":5920,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2026,"record_offset":9672,"parent_record_offset":9640,"ancestor_record_offset":5920,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2026,"record_offset":9952,"parent_record_offset":9924,"ancestor_record_offset":5920,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2026,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":5920,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":5920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":7424,"parent_record_offset":7388,"ancestor_record_offset":5920,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_entry_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":8196,"parent_record_offset":8152,"ancestor_record_offset":5920,"inlinee":7770,"source_line":1873,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":6940,"parent_record_offset":6616,"ancestor_record_offset":5920,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6404,"parent_record_offset":6292,"ancestor_record_offset":5920,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6248,"parent_record_offset":6208,"ancestor_record_offset":5920,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6292,"parent_record_offset":6248,"ancestor_record_offset":5920,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6332,"parent_record_offset":6292,"ancestor_record_offset":5920,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":6616,"parent_record_offset":6584,"ancestor_record_offset":5920,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::raw::impl$14::deserialize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/raw.rs
//     evidence={"module":2026,"record_offset":6524,"parent_record_offset":6208,"ancestor_record_offset":5920,"inlinee":7670,"source_line":481,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":6428,"parent_record_offset":6404,"ancestor_record_offset":5920,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":6360,"parent_record_offset":6332,"ancestor_record_offset":5920,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2026,"record_offset":264,"rva":31071136,"va":5399780256}
// 编译实例 RVAs: 0x1da1ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17068074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e82a700401","size":5,"soft_fail":false,"target_va":5416848880,"va":5399780801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-24102180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8dc3a90fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399780879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29626670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8d2ee3bfe","size":5,"soft_fail":false,"target_va":5370153648,"va":5399780313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29626885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8fbed3bfe","size":5,"soft_fail":false,"target_va":5370154240,"va":5399781120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030e630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27866590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e822ca56fe","size":5,"soft_fail":false,"target_va":5371913776,"va":5399780361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d82260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16907090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8ae04fefe","size":5,"soft_fail":false,"target_va":5382873696,"va":5399780781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d82420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16906700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e83406fefe","size":5,"soft_fail":false,"target_va":5382874144,"va":5399780839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16898836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8ec24fefe","size":5,"soft_fail":false,"target_va":5382881904,"va":5399780735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da31f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e821130000","size":5,"soft_fail":false,"target_va":5399785968,"va":5399781066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8d5120000","size":5,"soft_fail":false,"target_va":5399786496,"va":5399781670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=5
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16542636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8ac6bfc00","size":5,"soft_fail":false,"target_va":5416323136,"va":5399780495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::hab7539d1bf208f16 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e89df8ffff","size":5,"soft_fail":false,"target_va":5399779808,"va":5399781694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$alloc..string..String$C$serde_json..value..Value$C$alloc..alloc..Global$GT$$GT$::h065e14f06e7b32d2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e87be3ffff","size":5,"soft_fail":false,"target_va":5399775408,"va":5399782704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$serde_json..value..Value$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e85cf5ffff","size":5,"soft_fail":false,"target_va":5399779152,"va":5399781871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e8fdf6ffff","size":5,"soft_fail":false,"target_va":5399779584,"va":5399781886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h64cf2b524f0eadd2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399780256,"raw_hex":"e821f1ffff","size":5,"soft_fail":false,"target_va":5399777712,"va":5399781514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":1556,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2026,"record_offset":1492,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":2668,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7769,"source_line":1927,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":1012,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":492,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":548,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":2026,"record_offset":2292,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11167,"source_line":127,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":2026,"record_offset":984,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11166,"source_line":41,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2026,"record_offset":1708,"parent_record_offset":1652,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2026,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":264,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":2026,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":264,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":2026,"record_offset":2048,"parent_record_offset":2020,"ancestor_record_offset":264,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2026,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2026,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":264,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2026,"record_offset":4276,"parent_record_offset":4244,"ancestor_record_offset":264,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2026,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":264,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2026,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":264,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2026,"record_offset":1940,"parent_record_offset":1904,"ancestor_record_offset":264,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_entry_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":2712,"parent_record_offset":2668,"ancestor_record_offset":264,"inlinee":7770,"source_line":1873,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2026,"record_offset":1456,"parent_record_offset":1044,"ancestor_record_offset":264,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":776,"parent_record_offset":632,"ancestor_record_offset":264,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":588,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":632,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":672,"parent_record_offset":632,"ancestor_record_offset":264,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":2026,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":264,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::raw::impl$14::deserialize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/raw.rs
//     evidence={"module":2026,"record_offset":952,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":7670,"source_line":481,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":264,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":264,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":2026,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":264,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/02026.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: std::io::cursor::impl$5::read_to_string<alloc::vec::Vec<u8,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3567,"record_offset":3476,"rva":40907104,"va":5409616224}
// 编译实例 RVAs: 0x2703160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String>::try_reserve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24831329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409616224,"raw_hex":"e89f1a85fe","size":5,"soft_fail":false,"target_va":5384784992,"va":5409616316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6736588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409616224,"raw_hex":"e8ccca6600","size":5,"soft_fail":false,"target_va":5416353040,"va":5409616447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7440682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409616224,"raw_hex":"e82a897100","size":5,"soft_fail":false,"target_va":5417057056,"va":5409616369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24786199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409616224,"raw_hex":"e8e9ca85fe","size":5,"soft_fail":false,"target_va":5384830080,"va":5409616274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3567,"record_offset":3888,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3567,"record_offset":3828,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3567,"record_offset":3860,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3567,"record_offset":3656,"parent_record_offset":3476,"ancestor_record_offset":3476,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3567,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3476,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3567,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":3476,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3567,"record_offset":4108,"parent_record_offset":4076,"ancestor_record_offset":3476,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3567,"record_offset":4076,"parent_record_offset":4044,"ancestor_record_offset":3476,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3567,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":3476,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3567,"record_offset":3916,"parent_record_offset":3888,"ancestor_record_offset":3476,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3567,"record_offset":4224,"parent_record_offset":3972,"ancestor_record_offset":3476,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3567,"record_offset":4016,"parent_record_offset":3972,"ancestor_record_offset":3476,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3567,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":3476,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3567,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":3476,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3567,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":3476,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3567,"record_offset":4188,"parent_record_offset":3972,"ancestor_record_offset":3476,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3567,"record_offset":3712,"parent_record_offset":3656,"ancestor_record_offset":3476,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3567,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":3476,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3567,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":3476,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":1744,"rva":14217408,"va":5382926528}
// 编译实例 RVAs: 0xd8f0c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1732077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ed6d1a00","size":5,"soft_fail":false,"target_va":5384659264,"va":5382927182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1762741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8b5e51a00","size":5,"soft_fail":false,"target_va":5384689616,"va":5382926870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1729535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ff631a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382926700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1730439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e887671a00","size":5,"soft_fail":false,"target_va":5384657024,"va":5382926580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1729765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8e5641a00","size":5,"soft_fail":false,"target_va":5384656512,"va":5382926742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-7248150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ea6691ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382926849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16913288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e888130201","size":5,"soft_fail":false,"target_va":5399840272,"va":5382926979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16914887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8c7190201","size":5,"soft_fail":false,"target_va":5399841952,"va":5382927060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h796235da11270d01 (.llvm.17100869928342201643)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33923545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8d9a10502","size":5,"soft_fail":false,"target_va":5416850752,"va":5382927202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33924503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e897a50502","size":5,"soft_fail":false,"target_va":5416851392,"va":5382926884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33694283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e84b220202","size":5,"soft_fail":false,"target_va":5416621536,"va":5382927248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1903471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e86f0b1d00","size":5,"soft_fail":false,"target_va":5384830080,"va":5382926604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t212894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e89e3f0300","size":5,"soft_fail":false,"target_va":5383139568,"va":5382926669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1785647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e82f3f1b00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382926652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1797400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8186d1b00","size":5,"soft_fail":false,"target_va":5384724448,"va":5382927043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":683,"record_offset":4356,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":3808,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":3752,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":1896,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":3868,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":3384,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":3356,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3244,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3904,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::deleted_mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2032,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":45513,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1744,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1744,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1744,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2728,"parent_record_offset":2664,"ancestor_record_offset":1744,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2696,"parent_record_offset":2664,"ancestor_record_offset":1744,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":1744,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1744,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":683,"record_offset":4392,"parent_record_offset":4356,"ancestor_record_offset":1744,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":3296,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":1744,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":683,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":1744,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2156,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2192,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":2468,"parent_record_offset":2032,"ancestor_record_offset":1744,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1744,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":2128,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":1940,"parent_record_offset":1920,"ancestor_record_offset":1744,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3324,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1744,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1744,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":1744,"inlinee":45518,"source_line":103,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1744,"inlinee":45519,"source_line":104,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2092,"parent_record_offset":2032,"ancestor_record_offset":1744,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":1744,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::shelve_mod::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1744,"inlinee":45511,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::read_id_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":9956,"rva":28617488,"va":5397326608}
// 编译实例 RVAs: 0x1b4ab10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12717558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e80af23dff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397326961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21649035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e875a9b5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397327718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-21648351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e821acb5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397327050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28605512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8b8834bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397327699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27988507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8e5ed54fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397327350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27985220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8bcfa54fe","size":5,"soft_fail":false,"target_va":5369343504,"va":5397328719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d81bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14454701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8537023ff","size":5,"soft_fail":false,"target_va":5382872048,"va":5397326744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84050
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14447573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e82b8c23ff","size":5,"soft_fail":false,"target_va":5382881360,"va":5397328928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ad9ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-462357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8ebf1f8ff","size":5,"soft_fail":false,"target_va":5396864752,"va":5397327104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2458530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8a2832500","size":5,"soft_fail":false,"target_va":5399786496,"va":5397327961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19729307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e89b0b2d01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397327744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::hab7539d1bf208f16 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2451812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e864692500","size":5,"soft_fail":false,"target_va":5399779808,"va":5397327991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19293637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8c5652601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397327152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$alloc..string..String$C$serde_json..value..Value$C$alloc..alloc..Global$GT$$GT$::h065e14f06e7b32d2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2446411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e84b542500","size":5,"soft_fail":false,"target_va":5399775408,"va":5397328992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h64cf2b524f0eadd2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2450428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8fc632500","size":5,"soft_fail":false,"target_va":5399777712,"va":5397327279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12574730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8f61f40ff","size":5,"soft_fail":false,"target_va":5384751936,"va":5397326661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::ModId::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e881080000","size":5,"soft_fail":false,"target_va":5397330224,"va":5397328042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":10444,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":11268,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":11356,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":10612,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":10500,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10360,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::get
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":11212,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":30733,"source_line":305,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":10108,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1793,"record_offset":10136,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":17357,"source_line":1005,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1793,"record_offset":10324,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":9956,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":9956,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":9956,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":9956,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":9956,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10868,"parent_record_offset":10844,"ancestor_record_offset":9956,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11812,"parent_record_offset":11660,"ancestor_record_offset":9956,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":9956,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1793,"record_offset":10764,"parent_record_offset":10736,"ancestor_record_offset":9956,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":13148,"parent_record_offset":13116,"ancestor_record_offset":9956,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":13272,"parent_record_offset":13244,"ancestor_record_offset":9956,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":9956,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":9956,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":9956,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":9956,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":9956,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12176,"parent_record_offset":12152,"ancestor_record_offset":9956,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11660,"parent_record_offset":11624,"ancestor_record_offset":9956,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":9956,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":9956,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11700,"parent_record_offset":11660,"ancestor_record_offset":9956,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11480,"parent_record_offset":11452,"ancestor_record_offset":9956,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11508,"parent_record_offset":11480,"ancestor_record_offset":9956,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11536,"parent_record_offset":11508,"ancestor_record_offset":9956,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":9956,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":9956,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":9956,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":11568,"parent_record_offset":11536,"ancestor_record_offset":9956,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":12100,"parent_record_offset":12068,"ancestor_record_offset":9956,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1793,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":9956,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1793,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":9956,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":9956,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":9956,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1793,"record_offset":11396,"parent_record_offset":11356,"ancestor_record_offset":9956,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":9956,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":11940,"parent_record_offset":11536,"ancestor_record_offset":9956,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":9956,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":10648,"parent_record_offset":10612,"ancestor_record_offset":9956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":13180,"parent_record_offset":13148,"ancestor_record_offset":9956,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":9956,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":9956,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":9956,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9956,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9956,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":9956,"inlinee":31538,"source_line":492,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":1793,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":9956,"inlinee":30728,"source_line":123,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10184,"parent_record_offset":10136,"ancestor_record_offset":9956,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":9956,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10256,"parent_record_offset":10216,"ancestor_record_offset":9956,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::io::error::windows::decode_error_kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\io\error\windows.rs
//     evidence={"module":1793,"record_offset":10292,"parent_record_offset":10136,"ancestor_record_offset":9956,"inlinee":17334,"source_line":13,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::read_id_file::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":9956,"inlinee":87456,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::legacy_texture::parse_legacy_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1778,"record_offset":1204,"rva":28465488,"va":5397174608}
// 编译实例 RVAs: 0x1b25950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12515655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8b90641ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5397174914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21496122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8c6feb7fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397174805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21496612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8dcfcb7fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397175311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28452605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e803d94dfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397174792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19882217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8e9602f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397174834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e891fdffff","size":5,"soft_fail":false,"target_va":5397174464,"va":5397175082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12380042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8761843ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397174901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19445551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e82fb72801","size":5,"soft_fail":false,"target_va":5416620794,"va":5397175238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19601959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8271a2b01","size":5,"soft_fail":false,"target_va":5416777152,"va":5397175188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h442f4b15ebd1d3f6 (.llvm.17100869928342201643)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19675103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8df372c01","size":5,"soft_fail":false,"target_va":5416850064,"va":5397174956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19674329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8d9342c01","size":5,"soft_fail":false,"target_va":5416849424,"va":5397175090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1864,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1778,"record_offset":2280,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":1812,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":1784,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::legacy_texture::parse_legacy_texture::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":1352,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":86291,"source_line":72,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1204,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1204,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1204,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":1204,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1204,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":1204,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2084,"parent_record_offset":2052,"ancestor_record_offset":1204,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2620,"parent_record_offset":2564,"ancestor_record_offset":1204,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1204,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":1204,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1204,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":1204,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1204,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2052,"parent_record_offset":2020,"ancestor_record_offset":1204,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1204,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":1204,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1896,"parent_record_offset":1864,"ancestor_record_offset":1204,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1928,"parent_record_offset":1896,"ancestor_record_offset":1204,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1204,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1778,"record_offset":2440,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1778,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1204,"inlinee":9708,"source_line":258,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1778,"record_offset":1372,"parent_record_offset":1352,"ancestor_record_offset":1204,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":1204,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1778,"record_offset":2212,"parent_record_offset":1928,"ancestor_record_offset":1204,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1778,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1204,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1778,"record_offset":2468,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1778,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1204,"inlinee":9709,"source_line":1711,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::legacy_texture::parse_legacy_texture::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1204,"inlinee":86292,"source_line":95,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::legacy_texture::parse_legacy_texture::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":2328,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":86293,"source_line":96,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zip::read::find_content<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3568,"record_offset":264,"rva":40907344,"va":5409616464}
// 编译实例 RVAs: 0x2703250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b98a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16030335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409616464,"raw_hex":"e881650bff","size":5,"soft_fail":false,"target_va":5393586336,"va":5409616666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u16
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":3568,"record_offset":944,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":45571,"source_line":112,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":3568,"record_offset":452,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3568,"record_offset":776,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3568,"record_offset":804,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3568,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3568,"record_offset":908,"parent_record_offset":828,"ancestor_record_offset":264,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3568,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":264,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3568,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":264,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3568,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":264,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3568,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3568,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":264,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3568,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":264,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3568,"record_offset":564,"parent_record_offset":508,"ancestor_record_offset":264,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3568,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3568,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3568,"record_offset":508,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":3568,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":264,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":3568,"record_offset":652,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/03568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2049,"record_offset":264,"rva":31134704,"va":5399843824}
// 编译实例 RVAs: 0x1db13f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15049327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8915d1aff","size":5,"soft_fail":false,"target_va":5384795984,"va":5399845306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15200068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8bc1018ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5399844975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24166238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8a2408ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5399844921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-24165536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e860438ffe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399844235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31122715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8e51a25fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5399844902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30505926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e83a842efe","size":5,"soft_fail":false,"target_va":5369338848,"va":5399844769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a770
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30504255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8c18a2efe","size":5,"soft_fail":false,"target_va":5369341808,"va":5399846058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30404694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8aa0f30fe","size":5,"soft_fail":false,"target_va":5369441232,"va":5399845921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21188437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8abb0bcfe","size":5,"soft_fail":false,"target_va":5378657872,"va":5399846304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d96210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16889499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e86549fefe","size":5,"soft_fail":false,"target_va":5382955536,"va":5399845030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8b65cffff","size":5,"soft_fail":false,"target_va":5399803536,"va":5399845333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142703380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9771558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8261a9500","size":5,"soft_fail":false,"target_va":5409616768,"va":5399845205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17211901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8fda10601","size":5,"soft_fail":false,"target_va":5417057056,"va":5399845150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16774775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e877f6ff00","size":5,"soft_fail":false,"target_va":5416620769,"va":5399845989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16556100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e844a0fc00","size":5,"soft_fail":false,"target_va":5416401488,"va":5399845383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16775906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8e2faff00","size":5,"soft_fail":false,"target_va":5416620794,"va":5399844883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..zip_archive..Shared$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14965240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e808a61bff","size":5,"soft_fail":false,"target_va":5384880992,"va":5399846227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15119262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e8624c19ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5399845945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9770260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e814159500","size":5,"soft_fail":false,"target_va":5409614144,"va":5399843879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::Zip64CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9770539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399843824,"raw_hex":"e82b169500","size":5,"soft_fail":false,"target_va":5409615152,"va":5399844608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2049,"record_offset":4416,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2049,"record_offset":4380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2049,"record_offset":4104,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2049,"record_offset":2864,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2049,"record_offset":4252,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2049,"record_offset":2240,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2049,"record_offset":500,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2049,"record_offset":532,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2049,"record_offset":3284,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":15377,"source_line":290,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] zip::read::unsupported_zip_error
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":2049,"record_offset":2836,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64039,"source_line":641,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::get_directory_counts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":2049,"record_offset":592,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64034,"source_line":297,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_direct] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":2049,"record_offset":560,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":64033,"source_line":29,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":3152,"parent_record_offset":2968,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2049,"record_offset":2420,"parent_record_offset":2352,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2049,"record_offset":4496,"parent_record_offset":4452,"ancestor_record_offset":264,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2049,"record_offset":4452,"parent_record_offset":4416,"ancestor_record_offset":264,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2388,"parent_record_offset":2352,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2968,"parent_record_offset":2936,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":2324,"parent_record_offset":2296,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2049,"record_offset":3008,"parent_record_offset":2968,"ancestor_record_offset":264,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2049,"record_offset":4140,"parent_record_offset":4104,"ancestor_record_offset":264,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2049,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2049,"record_offset":896,"parent_record_offset":848,"ancestor_record_offset":264,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2049,"record_offset":1460,"parent_record_offset":848,"ancestor_record_offset":264,"inlinee":64037,"source_line":321,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2049,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":264,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2049,"record_offset":3844,"parent_record_offset":3816,"ancestor_record_offset":264,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2049,"record_offset":3744,"parent_record_offset":3540,"ancestor_record_offset":264,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2049,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":264,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2049,"record_offset":4308,"parent_record_offset":4280,"ancestor_record_offset":264,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2049,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":264,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2049,"record_offset":812,"parent_record_offset":724,"ancestor_record_offset":264,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2049,"record_offset":4280,"parent_record_offset":4252,"ancestor_record_offset":264,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2049,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":264,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":264,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":3084,"parent_record_offset":3056,"ancestor_record_offset":264,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":2012,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":5654,"source_line":761,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":264,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":1940,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":8773,"source_line":917,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":264,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2049,"record_offset":3872,"parent_record_offset":3816,"ancestor_record_offset":264,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2049,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":264,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2049,"record_offset":3452,"parent_record_offset":3416,"ancestor_record_offset":264,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2049,"record_offset":2124,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2049,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":264,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2049,"record_offset":5040,"parent_record_offset":4956,"ancestor_record_offset":264,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2049,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2049,"record_offset":2068,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2049,"record_offset":4188,"parent_record_offset":4140,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2049,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":264,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2049,"record_offset":3660,"parent_record_offset":3540,"ancestor_record_offset":264,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2049,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":264,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2049,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":264,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2049,"record_offset":4216,"parent_record_offset":4140,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2049,"record_offset":1976,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2049,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":264,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2049,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":264,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2049,"record_offset":1012,"parent_record_offset":956,"ancestor_record_offset":264,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2049,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":264,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2049,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":264,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2049,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":264,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2049,"record_offset":5204,"parent_record_offset":5172,"ancestor_record_offset":264,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2049,"record_offset":5144,"parent_record_offset":5104,"ancestor_record_offset":264,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2049,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":264,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_capacity_and_hasher
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2049,"record_offset":3980,"parent_record_offset":3316,"ancestor_record_offset":264,"inlinee":13442,"source_line":499,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5472,"parent_record_offset":4868,"ancestor_record_offset":264,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5324,"parent_record_offset":5104,"ancestor_record_offset":264,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4956,"parent_record_offset":4900,"ancestor_record_offset":264,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":264,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":264,"inlinee":5350,"source_line":643,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":264,"inlinee":12660,"source_line":692,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5876,"parent_record_offset":5844,"ancestor_record_offset":264,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":264,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4868,"parent_record_offset":4808,"ancestor_record_offset":264,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":264,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5844,"parent_record_offset":4808,"ancestor_record_offset":264,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4840,"parent_record_offset":4808,"ancestor_record_offset":264,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4900,"parent_record_offset":4868,"ancestor_record_offset":264,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4056,"parent_record_offset":4028,"ancestor_record_offset":264,"inlinee":12648,"source_line":1694,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5908,"parent_record_offset":5876,"ancestor_record_offset":264,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":264,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2049,"record_offset":5076,"parent_record_offset":4868,"ancestor_record_offset":264,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2049,"record_offset":3360,"parent_record_offset":3316,"ancestor_record_offset":264,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2049,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":264,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2049,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2049,"record_offset":3816,"parent_record_offset":3416,"ancestor_record_offset":264,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2049,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":264,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2049,"record_offset":956,"parent_record_offset":924,"ancestor_record_offset":264,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2049,"record_offset":700,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2049,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":2049,"record_offset":1100,"parent_record_offset":924,"ancestor_record_offset":264,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2049,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":264,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2049,"record_offset":3540,"parent_record_offset":3508,"ancestor_record_offset":264,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2049,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":264,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2049,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] zip::read::impl$4::get_directory_counts::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":2049,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":264,"inlinee":64041,"source_line":337,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":2049,"record_offset":1904,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":64033,"source_line":29,"archive":"_pdb/C/modules/02049.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::Zip64CentralDirectoryEndLocator::parse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":2049,"record_offset":848,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":64035,"source_line":118,"archive":"_pdb/C/modules/02049.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]

// ------------------------------------------------------------------
// 函数: alloc::collections::btree::node::Handle::insert_recursing<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global> >
// 信息 ID: 2997c86604537d17793bc0c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["tg_mods::overlay::ModTextureBinding"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["tg_mods::overlay::ModTextureBinding"]; 字段布局 = unknown。
// [类型关系] alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0 -> ["alloc::string::String", "enum2$<tg_mods::overlay::ModTextureBinding>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] alloc::collections::btree::node::Handle::insert_recursing -> ["alloc::string::String", "enum2$<tg_mods::overlay::ModTextureBinding>", "alloc::alloc::Global", "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::collections::btree::map::entry::VacantEntry::insert_entry<alloc::string::String,enum2$<tg_mods::overlay::ModTextureBinding>,alloc::alloc::Global> @ 0x1da7b8e -> 0x1da8170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 信息 ID: a8dc85559a31570700ffa4f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] hashbrown::raw::RawTableInner::fallible_with_capacity -> ["alloc::alloc::Global"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > @ 0x1db18a6 -> 0xd96210 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,std::path::PathBuf>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,std::path::PathBuf,std::hash::random::RandomState> > @ 0x2d9f1bd -> 0xd96210 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,usize>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,usize,std::hash::random::RandomState> > @ 0x2d9f7dd -> 0xd96210 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_mods::identity::ModId,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState> > | hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState> > @ 0x2d9fd9d -> 0xd96210 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState> > @ 0x2da0341 -> 0xd96210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 信息 ID: c2741493b418804ff25d7b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d047 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d191 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> @ 0xd81e81 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd821ff -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd8210c -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> @ 0x2d9eda7 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> @ 0x2d9ef33 -> 0x15d890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
// 信息 ID: 1bf8e5fdc04130f6a6b6df0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x15cd07 -> 0x15dad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x15ce51 -> 0x15dad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 信息 ID: f282a6b86a913fdf1ac23c39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_integer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd83376 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd833bb -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa090f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa0944 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x115c9ca -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f856 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f89b -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e8ef -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e992 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2809 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2879 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8af9 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8b69 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de230f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de23b2 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_number<serde_json::read::StrRead>
// 信息 ID: c3d0dcef31a46acdb8f850bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_number -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x15d5ba -> 0x15ccc0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_str<serde_json::read::SliceRead,serde_json::raw::BoxedFromString>
// 信息 ID: 380a78f9b561a1b7528f84a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::impl$7::deserialize_str -> ["serde_json::read::SliceRead", "serde_json::raw::BoxedFromString"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::SliceRead> > @ 0x1da276f -> 0xd843c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_str<serde_json::read::StrRead,serde_json::raw::BoxedFromString>
// 信息 ID: 60f1a2c252b289b79a582bdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::impl$7::deserialize_str -> ["serde_json::read::StrRead", "serde_json::raw::BoxedFromString"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> > @ 0x1da1d7f -> 0xd84270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::SliceRead> >
// 信息 ID: 85f04d4c3db5c77efe5615e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::MapAccess -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// [类型关系] serde_json::value::de::impl$0::deserialize::impl$0::visit_map -> ["serde_json::de::MapAccess<serde_json::read::SliceRead>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> >
// 信息 ID: 2a6c5a25d4c8992a74f316eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::MapAccess -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// [类型关系] serde_json::value::de::impl$0::deserialize::impl$0::visit_map -> ["serde_json::de::MapAccess<serde_json::read::StrRead>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: std::io::cursor::impl$5::read_to_string<alloc::vec::Vec<u8,alloc::alloc::Global> >
// 信息 ID: dd1961e49e9d166516037e50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::impl$5::read_to_string -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod
// 信息 ID: 9ddf33f98974eebd08feb8ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::read_id_file
// 信息 ID: ab8bb49f87104ecee8a43966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::mod_at_root @ 0xd8f4d4 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::zip_mod_id @ 0x1b4a238 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::zip_mod_id @ 0x1b4a5a5 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::read_mod_id @ 0x1b4aaae -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::folder_mod_id @ 0x1b4b4c7 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::legacy_texture::parse_legacy_texture
// 信息 ID: 7cf88882d05cee217de9f388
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e42ec -> 0x1b25950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zip::read::find_content<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: 3cb0109f86c9beccd8d7c85c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zip::read::find_content -> ["std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zip::read::zip_archive::ZipArchive::by_index<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > @ 0x1db1f6a -> 0x2703250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: 10443a79a320617854fa2493
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zip::read::zip_archive::ZipArchive::new -> ["std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd91f88 -> 0x1db13f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
