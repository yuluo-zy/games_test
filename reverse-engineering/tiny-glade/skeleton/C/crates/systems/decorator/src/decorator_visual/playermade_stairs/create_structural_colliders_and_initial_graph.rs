// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/create_structural_colliders_and_initial_graph.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0140e43a410aa4c13bd3fec77237b6e99b0689ffd1ce89686231a36190d85819
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: create_structural_colliders_and_initial_graph.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 462, 1542
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bitvec; // namespace candidate
//   package identities: [{"name":"bitvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.13.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use tap; // namespace candidate
//   package identities: [{"name":"tap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use wyz; // namespace candidate
//   package identities: [{"name":"wyz","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":2028,"rva":9074912,"va":5377784032}
// 编译实例 RVAs: 0x8a78e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2107258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e886d8dfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377785941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2110563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e89dcbdfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377789262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9063739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8c5b275ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377785926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8319202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e81e0f81ff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377791053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4308339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e88d42beff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377788110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ccc0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4304767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e88150beff","size":5,"soft_fail":false,"target_va":5373480128,"va":5377784890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a577d0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1764599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8f7ec1a00","size":5,"soft_fail":false,"target_va":5379553232,"va":5377788628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4096207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8cf803e00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377791180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12663226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8ba39c100","size":5,"soft_fail":false,"target_va":5390451360,"va":5377788129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c1220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12684293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8058cc100","size":5,"soft_fail":false,"target_va":5390471712,"va":5377787414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414f5b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12900808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8c8d9c400","size":5,"soft_fail":false,"target_va":5390687024,"va":5377786211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38564726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e876734c02","size":5,"soft_fail":false,"target_va":5416353040,"va":5377788309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39271081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8a93a5702","size":5,"soft_fail":false,"target_va":5417057056,"va":5377785970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38831189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e855845002","size":5,"soft_fail":false,"target_va":5416620769,"va":5377789575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::slice::api::impl$2::index::closure$0<usize,bitvec::order::Lsb0>
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t38592135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e887de4c02","size":5,"soft_fail":false,"target_va":5416382096,"va":5377789956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38831789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8ad865002","size":5,"soft_fail":false,"target_va":5416621408,"va":5377789614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38832391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e807895002","size":5,"soft_fail":false,"target_va":5416621600,"va":5377789204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38831629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e80d865002","size":5,"soft_fail":false,"target_va":5416621188,"va":5377789554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38831833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8d9865002","size":5,"soft_fail":false,"target_va":5416621762,"va":5377789924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38832335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8cf885002","size":5,"soft_fail":false,"target_va":5416622240,"va":5377789900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38832842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8ca8a5002","size":5,"soft_fail":false,"target_va":5416622320,"va":5377789473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8221e0000","size":5,"soft_fail":false,"target_va":5377791856,"va":5377784137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38831295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8bf845002","size":5,"soft_fail":false,"target_va":5416621376,"va":5377790076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::graph
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1764365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e80dec1a00","size":5,"soft_fail":false,"target_va":5379552272,"va":5377787902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_span
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3869717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8150c3b00","size":5,"soft_fail":false,"target_va":5381657232,"va":5377787510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_span
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3869960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8080d3b00","size":5,"soft_fail":false,"target_va":5381656256,"va":5377786291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMesh::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1370516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e894e91400","size":5,"soft_fail":false,"target_va":5379156544,"va":5377786023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6179305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8e9495e00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377784482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6187812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8246b5e00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377784407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6175580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e85c3b5e00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377789327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6178704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e890475e00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377784107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t38928067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8c3fe5102","size":5,"soft_fail":false,"target_va":5416717696,"va":5377789624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38829710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e88e7e5002","size":5,"soft_fail":false,"target_va":5416619360,"va":5377789645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::calculate_adjusted_ladder_positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e88ba3ffff","size":5,"soft_fail":false,"target_va":5377761904,"va":5377785568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e8b1f5ffff","size":5,"soft_fail":false,"target_va":5377783024,"va":5377785658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::IsoTransform::from_translation
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5487496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377784032,"raw_hex":"e888bb5300","size":5,"soft_fail":false,"target_va":5383273632,"va":5377786131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":462,"record_offset":8076,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":12872,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":12508,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":6288,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13632,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":462,"record_offset":8344,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bitvec::slice::ops::impl$3::index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/ops.rs
//     evidence={"module":462,"record_offset":7332,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":27322,"source_line":158,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":462,"record_offset":4904,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::ops::impl$15::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/ops.rs
//     evidence={"module":462,"record_offset":6520,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":27320,"source_line":236,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":462,"record_offset":14388,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":462,"record_offset":4680,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":462,"record_offset":15572,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":462,"record_offset":10512,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":10400,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":15508,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":22886,"source_line":917,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":15536,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":16956,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":4596,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":6188,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":6108,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":462,"record_offset":15812,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":8456,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":462,"record_offset":8540,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":462,"record_offset":12900,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":12046,"source_line":454,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::nodes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":462,"record_offset":12536,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":13870,"source_line":428,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::impl$55::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/macros.rs
//     evidence={"module":462,"record_offset":12704,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":13874,"source_line":37,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":3788,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":2388,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":2776,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$50::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":462,"record_offset":5928,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":12069,"source_line":2307,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":14516,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":10484,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":462,"record_offset":4960,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":2028,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":2028,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":2028,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":2028,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":2028,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":2028,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":8188,"parent_record_offset":8156,"ancestor_record_offset":2028,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":17116,"parent_record_offset":17028,"ancestor_record_offset":2028,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":462,"record_offset":8156,"parent_record_offset":8112,"ancestor_record_offset":2028,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":462,"record_offset":8112,"parent_record_offset":8076,"ancestor_record_offset":2028,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":2028,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":2028,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":2028,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":2028,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":2028,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":2028,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":14256,"parent_record_offset":14232,"ancestor_record_offset":2028,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":462,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":2028,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":462,"record_offset":20852,"parent_record_offset":20816,"ancestor_record_offset":2028,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":462,"record_offset":20816,"parent_record_offset":20788,"ancestor_record_offset":2028,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13892,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":2028,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13100,"parent_record_offset":13072,"ancestor_record_offset":2028,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":2028,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":14208,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":14012,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":13072,"parent_record_offset":13044,"ancestor_record_offset":2028,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":462,"record_offset":13656,"parent_record_offset":13632,"ancestor_record_offset":2028,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":462,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":2028,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitIdx::offset
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":462,"record_offset":6924,"parent_record_offset":6896,"ancestor_record_offset":2028,"inlinee":27269,"source_line":259,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::proxy::BitRef::from_bitptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/proxy.rs
//     evidence={"module":462,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":2028,"inlinee":27307,"source_line":84,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":462,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":2028,"inlinee":27281,"source_line":706,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::as_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":462,"record_offset":7520,"parent_record_offset":7404,"ancestor_record_offset":2028,"inlinee":27480,"source_line":496,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::offset
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":462,"record_offset":6896,"parent_record_offset":6808,"ancestor_record_offset":2028,"inlinee":27268,"source_line":545,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::read
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":462,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":2028,"inlinee":27308,"source_line":775,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::span_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":462,"record_offset":7128,"parent_record_offset":6784,"ancestor_record_offset":2028,"inlinee":27282,"source_line":245,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":462,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":2028,"inlinee":4818,"source_line":251,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::head
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":462,"record_offset":6856,"parent_record_offset":6808,"ancestor_record_offset":2028,"inlinee":27252,"source_line":293,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":462,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":2028,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::new_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":462,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":2028,"inlinee":27283,"source_line":200,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::to_bitptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":462,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":2028,"inlinee":27266,"source_line":532,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::as_bitptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice.rs
//     evidence={"module":462,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":2028,"inlinee":27479,"source_line":449,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6648,"parent_record_offset":6584,"ancestor_record_offset":2028,"inlinee":27275,"source_line":87,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$2::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":2028,"inlinee":27324,"source_line":2559,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$2::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":2028,"inlinee":27325,"source_line":2579,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$2::index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":2028,"inlinee":27323,"source_line":2592,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6704,"parent_record_offset":6584,"ancestor_record_offset":2028,"inlinee":27473,"source_line":2623,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":2028,"inlinee":27475,"source_line":2714,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6760,"parent_record_offset":6704,"ancestor_record_offset":2028,"inlinee":27476,"source_line":2648,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::get_unchecked::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":2028,"inlinee":27478,"source_line":2722,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":2028,"inlinee":27472,"source_line":2660,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$7::index::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":462,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":2028,"inlinee":27482,"source_line":2663,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::ops::impl$11::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/ops.rs
//     evidence={"module":462,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":2028,"inlinee":27470,"source_line":178,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2028,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":2028,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2028,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2028,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4204,"parent_record_offset":4088,"ancestor_record_offset":2028,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":462,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":2028,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":8644,"parent_record_offset":8608,"ancestor_record_offset":2028,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":2028,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":2028,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":2028,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":2028,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":5752,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":15604,"parent_record_offset":15572,"ancestor_record_offset":2028,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":462,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":2028,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":462,"record_offset":8812,"parent_record_offset":8788,"ancestor_record_offset":2028,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":462,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":2028,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":462,"record_offset":8676,"parent_record_offset":8608,"ancestor_record_offset":2028,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":462,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":2028,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":462,"record_offset":8580,"parent_record_offset":8540,"ancestor_record_offset":2028,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":462,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":2028,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":462,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2028,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":6252,"parent_record_offset":6216,"ancestor_record_offset":2028,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":14180,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":2028,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":17084,"parent_record_offset":17056,"ancestor_record_offset":2028,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":462,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":2028,"inlinee":27270,"source_line":2100,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":5484,"parent_record_offset":5456,"ancestor_record_offset":2028,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":5456,"parent_record_offset":5428,"ancestor_record_offset":2028,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":462,"record_offset":3936,"parent_record_offset":3896,"ancestor_record_offset":2028,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":462,"record_offset":4804,"parent_record_offset":4704,"ancestor_record_offset":2028,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::impl$64::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":462,"record_offset":6620,"parent_record_offset":6584,"ancestor_record_offset":2028,"inlinee":12115,"source_line":354,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":12836,"parent_record_offset":12728,"ancestor_record_offset":2028,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":4860,"parent_record_offset":4832,"ancestor_record_offset":2028,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":2028,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":7264,"parent_record_offset":6584,"ancestor_record_offset":2028,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":2028,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":4552,"parent_record_offset":3896,"ancestor_record_offset":2028,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":2028,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":7188,"parent_record_offset":7152,"ancestor_record_offset":2028,"inlinee":9483,"source_line":1053,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":2028,"inlinee":9482,"source_line":463,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":2028,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":4268,"parent_record_offset":4036,"ancestor_record_offset":2028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":2028,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":2028,"inlinee":7420,"source_line":352,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":14112,"parent_record_offset":14088,"ancestor_record_offset":2028,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":2028,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":13856,"parent_record_offset":13752,"ancestor_record_offset":2028,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":20884,"parent_record_offset":20852,"ancestor_record_offset":2028,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":2028,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":14088,"parent_record_offset":14064,"ancestor_record_offset":2028,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2028,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":462,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":2028,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":462,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":2028,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":2028,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":2028,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":6480,"parent_record_offset":6444,"ancestor_record_offset":2028,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":6316,"parent_record_offset":6288,"ancestor_record_offset":2028,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":2028,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":462,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":2028,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2028,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2028,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":2028,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":2028,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":4740,"parent_record_offset":4704,"ancestor_record_offset":2028,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":15688,"parent_record_offset":15572,"ancestor_record_offset":2028,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":2028,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":462,"record_offset":3100,"parent_record_offset":2984,"ancestor_record_offset":2028,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":462,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":2028,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":462,"record_offset":2812,"parent_record_offset":2776,"ancestor_record_offset":2028,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":462,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":2028,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":462,"record_offset":2920,"parent_record_offset":2884,"ancestor_record_offset":2028,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":462,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2028,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":462,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2028,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":462,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2028,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":462,"record_offset":3424,"parent_record_offset":2884,"ancestor_record_offset":2028,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2492,"parent_record_offset":2460,"ancestor_record_offset":2028,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":2028,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":2028,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2672,"parent_record_offset":2616,"ancestor_record_offset":2028,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":2028,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2432,"parent_record_offset":2388,"ancestor_record_offset":2028,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":462,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":2028,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":462,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2028,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":462,"record_offset":3268,"parent_record_offset":2984,"ancestor_record_offset":2028,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":462,"record_offset":3240,"parent_record_offset":2984,"ancestor_record_offset":2028,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":462,"record_offset":3044,"parent_record_offset":2984,"ancestor_record_offset":2028,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":462,"record_offset":20912,"parent_record_offset":20852,"ancestor_record_offset":2028,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":462,"record_offset":20944,"parent_record_offset":20816,"ancestor_record_offset":2028,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":462,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":2028,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":462,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":2028,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":462,"record_offset":13256,"parent_record_offset":12972,"ancestor_record_offset":2028,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":2028,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":5428,"parent_record_offset":5396,"ancestor_record_offset":2028,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":5368,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":5228,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":5780,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":5312,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":462,"record_offset":5172,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":462,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":2028,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":462,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":2028,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":462,"record_offset":14544,"parent_record_offset":14516,"ancestor_record_offset":2028,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":462,"record_offset":5636,"parent_record_offset":5528,"ancestor_record_offset":2028,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":462,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":2028,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":2028,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5872,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":2028,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":2028,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5068,"parent_record_offset":5008,"ancestor_record_offset":2028,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":2028,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5200,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":462,"record_offset":5528,"parent_record_offset":5116,"ancestor_record_offset":2028,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":462,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":2028,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::inner::Core::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/inner.rs
//     evidence={"module":462,"record_offset":13044,"parent_record_offset":13016,"ancestor_record_offset":2028,"inlinee":9868,"source_line":125,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::as_entries
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":462,"record_offset":13016,"parent_record_offset":12972,"ancestor_record_offset":2028,"inlinee":9867,"source_line":193,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":462,"record_offset":12936,"parent_record_offset":12900,"ancestor_record_offset":2028,"inlinee":9866,"source_line":838,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::get_index_of
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":462,"record_offset":12972,"parent_record_offset":12936,"ancestor_record_offset":2028,"inlinee":14846,"source_line":885,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":462,"record_offset":13312,"parent_record_offset":12972,"ancestor_record_offset":2028,"inlinee":14822,"source_line":819,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::keys
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":462,"record_offset":12560,"parent_record_offset":12536,"ancestor_record_offset":2028,"inlinee":13871,"source_line":251,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::iter::Keys::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map/iter.rs
//     evidence={"module":462,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":2028,"inlinee":13873,"source_line":334,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::iter::impl$39::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/macros.rs
//     evidence={"module":462,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":2028,"inlinee":13875,"source_line":155,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":462,"record_offset":8740,"parent_record_offset":8540,"ancestor_record_offset":2028,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":462,"record_offset":13724,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":11051,"source_line":719,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":462,"record_offset":13828,"parent_record_offset":13752,"ancestor_record_offset":2028,"inlinee":11054,"source_line":725,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::size_hint
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":462,"record_offset":14040,"parent_record_offset":13680,"ancestor_record_offset":2028,"inlinee":11052,"source_line":728,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":3824,"parent_record_offset":3788,"ancestor_record_offset":2028,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":17308,"parent_record_offset":17276,"ancestor_record_offset":2028,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":17788,"parent_record_offset":17572,"ancestor_record_offset":2028,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":462,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":2028,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":4036,"parent_record_offset":3896,"ancestor_record_offset":2028,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":462,"record_offset":4240,"parent_record_offset":4036,"ancestor_record_offset":2028,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":462,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":2028,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":462,"record_offset":5984,"parent_record_offset":5952,"ancestor_record_offset":2028,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":462,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":2028,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":462,"record_offset":3992,"parent_record_offset":3964,"ancestor_record_offset":2028,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":462,"record_offset":3896,"parent_record_offset":3860,"ancestor_record_offset":2028,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":462,"record_offset":3860,"parent_record_offset":3824,"ancestor_record_offset":2028,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/create_structural_colliders_and_initial_graph.rs
//     evidence={"module":462,"record_offset":4832,"parent_record_offset":4804,"ancestor_record_offset":2028,"inlinee":27468,"source_line":55,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] tap::pipe::Pipe::pipe
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"tap","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tap-1.0.1/src/pipe.rs
//     evidence={"module":462,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":2028,"inlinee":13907,"source_line":73,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::offset
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":462,"record_offset":6980,"parent_record_offset":6896,"ancestor_record_offset":2028,"inlinee":27271,"source_line":256,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::with_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":462,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":2028,"inlinee":16744,"source_line":328,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::impl$10::offset::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":462,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":2028,"inlinee":16735,"source_line":259,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B4C
// 首条位置: {"module":462,"inlinee":27468}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":4860,"parent_record_offset":4832,"ancestor_record_offset":4832,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider
// 记录形式: compiled_procedure; instances=1
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":272,"rva":9073904,"va":5377783024}
// 编译实例 RVAs: 0x8a74f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: create_structural_colliders_and_initial_graph.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5541820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377783024,"raw_hex":"e8bc8f5400","size":5,"soft_fail":false,"target_va":5383325024,"va":5377783199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_convex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1383022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377783024,"raw_hex":"e86e1a1500","size":5,"soft_fail":false,"target_va":5379166768,"va":5377783741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_sphere
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1383711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377783024,"raw_hex":"e81f1d1500","size":5,"soft_fail":false,"target_va":5379167632,"va":5377783916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":462,"record_offset":1784,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":624,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":504,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26814,"source_line":1513,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":1232,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":944,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":560,"parent_record_offset":532,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":272,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":1928,"parent_record_offset":1816,"ancestor_record_offset":272,"inlinee":26998,"source_line":1641,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/create_structural_colliders_and_initial_graph.rs
//     evidence={"module":462,"record_offset":1816,"parent_record_offset":1784,"ancestor_record_offset":272,"inlinee":27466,"source_line":292,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B4A
// 首条位置: {"module":462,"inlinee":27466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":462,"record_offset":1928,"parent_record_offset":1816,"ancestor_record_offset":1816,"inlinee":26998,"source_line":1641,"archive":"_pdb/C/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":1816,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00462.jsonl.gz"}
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
// 完整结构及原始类型记录: create_structural_colliders_and_initial_graph.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: create_structural_colliders_and_initial_graph.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: create_structural_colliders_and_initial_graph.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph
// 信息 ID: 56381cde016b9388978da301
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 84418b6ea985a69dfed69910: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x182112 -> 0x8a78e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x29ea7e -> 0x8a78e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph::closure$0
// 信息 ID: 0751f6e79bffa13531fef16b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph; module=462, inline_record=4832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider
// 信息 ID: df4e8ae00bb869a498e0a8ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph @ 0x8a7f3a -> 0x8a74f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::create_structural_colliders_and_initial_graph @ 0x8a82ea -> 0x8a74f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider::closure$0
// 信息 ID: 7aea113168a8e9dae0ce35dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::create_structural_colliders_and_initial_graph::gen_stairs_structural_collider; module=462, inline_record=1816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
