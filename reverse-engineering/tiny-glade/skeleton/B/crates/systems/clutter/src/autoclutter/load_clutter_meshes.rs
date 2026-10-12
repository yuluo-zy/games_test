// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ed6c495cfaea8c765630857a0005719428c2b3e22616354ed1a802b24a60a8f3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_clutter_meshes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3785, 3796
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 0, 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB08
// 首条位置: {"module":3785,"record_offset":744,"rva":36445376,"va":5405154496}
// 编译实例 RVAs: 0x22c1cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30073236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e86c1e35fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405154719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed4b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20894969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8072bc1fe","size":5,"soft_fail":false,"target_va":5384260384,"va":5405155348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142607f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3432770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e842613400","size":5,"soft_fail":false,"target_va":5408587568,"va":5405154793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6385797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e885706100","size":5,"soft_fail":false,"target_va":5411540992,"va":5405155190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22500378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8e6aba8fe","size":5,"soft_fail":false,"target_va":5382655024,"va":5405155397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6387637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8b5776100","size":5,"soft_fail":false,"target_va":5411542880,"va":5405155238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25928704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8005c74fe","size":5,"soft_fail":false,"target_va":5379226224,"va":5405154923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_radius_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25926777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8876374fe","size":5,"soft_fail":false,"target_va":5379228128,"va":5405154900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24801856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8c08d85fe","size":5,"soft_fail":false,"target_va":5380353184,"va":5405155035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23239182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e8f2659dfe","size":5,"soft_fail":false,"target_va":5381915520,"va":5405154697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23220151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405154496,"raw_hex":"e849b09dfe","size":5,"soft_fail":false,"target_va":5381934464,"va":5405154610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3785,"record_offset":912,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3785,"record_offset":1096,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":744,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":744,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":744,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":972,"parent_record_offset":940,"ancestor_record_offset":744,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":1140,"parent_record_offset":1096,"ancestor_record_offset":744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":744,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3785,"record_offset":260,"rva":36439904,"va":5405149024}
// 编译实例 RVAs: 0x22c0760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30072974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405149024,"raw_hex":"e8721f35fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405154473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cea830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6120257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405149024,"raw_hex":"e8bf9ca2ff","size":5,"soft_fail":false,"target_va":5399029808,"va":5405150060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405149024,"raw_hex":"e88ea10000","size":5,"soft_fail":false,"target_va":5405195744,"va":5405154381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
//     owner=package_namespace_candidate; count=55
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405149024,"raw_hex":"e8d9140000","size":5,"soft_fail":false,"target_va":5405154496,"va":5405149154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3785,"record_offset":420,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":504,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3785,"record_offset":668,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3785,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3785,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3785,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3785,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":260,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3785,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":260,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3785,"record_offset":636,"parent_record_offset":604,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3785,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03785.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB1F
// 首条位置: {"module":3796,"inlinee":121631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3796,"record_offset":5156,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3796,"record_offset":4964,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3796,"record_offset":5128,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3796,"record_offset":5648,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs
//     evidence={"module":3796,"record_offset":5324,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":121608,"source_line":24,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4908,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":4908,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4908,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4908,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4908,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3796,"record_offset":5800,"parent_record_offset":5744,"ancestor_record_offset":4908,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3796,"record_offset":5460,"parent_record_offset":5324,"ancestor_record_offset":4908,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3796,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":4908,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3796,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":4908,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3796,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":4908,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3796,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":4908,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3796,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":4908,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3796,"record_offset":5428,"parent_record_offset":5388,"ancestor_record_offset":4908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3796,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":4908,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03796.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3796,"record_offset":5388,"parent_record_offset":5324,"ancestor_record_offset":4908,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03796.jsonl.gz"}
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
// 完整结构及原始类型记录: load_clutter_meshes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_clutter_meshes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_clutter_meshes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes
// 信息 ID: b8f7984af13a5712e0fb9e21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 01032eead5fe87cdf7af3db3: void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)> > @ 0x1cb2161 -> 0x22c0760 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x1cde8c6 -> 0x22c0760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
// 信息 ID: 9205bc49ed67512edc4c015e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// 引用方: 原程序直接转移 55 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c07e2 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c082f -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c087c -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c08c9 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0916 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0963 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c09b0 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c09fd -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0a4a -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0a97 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0ae4 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0b31 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0bba -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0c0f -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0c64 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x22c0cb9 -> 0x22c1cc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$20::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; module=3796, inline_record=5324; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$1
// 信息 ID: 79d86a201a8531fde5cb7788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::resources::clutter_meshes::ClutterMesh>,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$20::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$20::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::try_trait::impl$0::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$20::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; module=3796, inline_record=4908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
