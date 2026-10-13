// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/collision/intersect_meshes.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 784812acf5947efb094b4634b96ac3aabe6299a9d457e319dc92ebfa8a8bea38
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: intersect_meshes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 395, 397, 406, 409, 448, 1268, 1308, 1325, 1378, 1397
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.12.3"},{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"PDB_registry_path_only","version":"1.9.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"PDB_registry_path_only","version":"0.17.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"PDB_registry_path_only","version":"0.6.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rapier3d; // namespace candidate
//   package identities: []
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::should_intersect_collider_meshes
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":397,"record_offset":33416,"rva":5497152,"va":5374206272}
// 编译实例 RVAs: 0x53e140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28982344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374206272,"raw_hex":"e8483cba01","size":5,"soft_fail":false,"target_va":5403188848,"va":5374206499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28772698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374206272,"raw_hex":"e85a09b701","size":5,"soft_fail":false,"target_va":5402979152,"va":5374206449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] slotmap::KeyData::from_ffi
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10739992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374206272,"raw_hex":"e818e1a300","size":5,"soft_fail":false,"target_va":5384946288,"va":5374206291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":397,"record_offset":34088,"parent_record_offset":33416,"ancestor_record_offset":33416,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":33816,"parent_record_offset":33416,"ancestor_record_offset":33416,"inlinee":22782,"source_line":1178,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":33596,"parent_record_offset":33416,"ancestor_record_offset":33416,"inlinee":22775,"source_line":210,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":397,"record_offset":33624,"parent_record_offset":33416,"ancestor_record_offset":33416,"inlinee":22777,"source_line":863,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":33752,"parent_record_offset":33656,"ancestor_record_offset":33416,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":33688,"parent_record_offset":33656,"ancestor_record_offset":33416,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":33716,"parent_record_offset":33688,"ancestor_record_offset":33416,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":397,"record_offset":33656,"parent_record_offset":33624,"ancestor_record_offset":33416,"inlinee":22778,"source_line":538,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":397,"record_offset":33780,"parent_record_offset":33752,"ancestor_record_offset":33416,"inlinee":22780,"source_line":542,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6497
// 首条位置: {"module":448,"inlinee":25751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":448,"record_offset":11992,"parent_record_offset":11964,"ancestor_record_offset":11964,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$22::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":448,"record_offset":12020,"parent_record_offset":11992,"ancestor_record_offset":11964,"inlinee":17657,"source_line":1469,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1268,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":11088,"inlinee":8696,"source_line":1469,"archive":"_pdb/A/modules/01268.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":448,"record_offset":12048,"parent_record_offset":11992,"ancestor_record_offset":11964,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$7::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B2C
// 首条位置: {"module":406,"inlinee":23340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::tuple::impl$15::partial_cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":406,"record_offset":63036,"parent_record_offset":63012,"ancestor_record_offset":63012,"inlinee":23342,"source_line":72,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$56::partial_cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63060,"parent_record_offset":63036,"ancestor_record_offset":63012,"inlinee":23343,"source_line":1549,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$57::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63084,"parent_record_offset":63060,"ancestor_record_offset":63012,"inlinee":23345,"source_line":1565,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::partial_cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63140,"parent_record_offset":63116,"ancestor_record_offset":63012,"inlinee":12578,"source_line":1549,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63164,"parent_record_offset":63140,"ancestor_record_offset":63012,"inlinee":12580,"source_line":1565,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$15::partial_cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":406,"record_offset":63116,"parent_record_offset":63036,"ancestor_record_offset":63012,"inlinee":23342,"source_line":72,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFB77
// 首条位置: {"module":1378,"inlinee":64375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":12424,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":12424,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":12424,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":12424,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12528,"parent_record_offset":12448,"ancestor_record_offset":12424,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1378,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":12424,"inlinee":17875,"source_line":806,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1378,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":12424,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01378.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58D0
// 首条位置: {"module":397,"inlinee":22736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$12::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5886
// 首条位置: {"module":395,"inlinee":22662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":397,"record_offset":7064,"rva":5483984,"va":5374193104}
// 编译实例 RVAs: 0x53add0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4583051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e87511baff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374196566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1570013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8230be8ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374195032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1446474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8b6ede9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374196165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1446085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e83befe9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374195792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405673a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t179117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8adbb0200","size":5,"soft_fail":false,"target_va":5374374816,"va":5374195694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602f20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t813389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e84d690c00","size":5,"soft_fail":false,"target_va":5375012640,"va":5374199246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140604540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t819136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8c07f0c00","size":5,"soft_fail":false,"target_va":5375018304,"va":5374199163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1229420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e86cc21200","size":5,"soft_fail":false,"target_va":5375423344,"va":5374193919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c1680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1594024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8a8521800","size":5,"soft_fail":false,"target_va":5375792768,"va":5374198739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1679648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e820a11900","size":5,"soft_fail":false,"target_va":5375881344,"va":5374201691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1686476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8ccbb1900","size":5,"soft_fail":false,"target_va":5375888160,"va":5374201679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dd9e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1708615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e847121a00","size":5,"soft_fail":false,"target_va":5375908320,"va":5374199700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceee30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8076249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8d93b7b00","size":5,"soft_fail":false,"target_va":5382270512,"va":5374194258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cef590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8072726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8162e7b00","size":5,"soft_fail":false,"target_va":5382272400,"va":5374199669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d6ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8598855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e847358300","size":5,"soft_fail":false,"target_va":5382794816,"va":5374195956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d74720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8617239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8177d8300","size":5,"soft_fail":false,"target_va":5382817568,"va":5374200324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9114502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e886138b00","size":5,"soft_fail":false,"target_va":5383308144,"va":5374193637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5bad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10618973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e85d08a200","size":5,"soft_fail":false,"target_va":5384813264,"va":5374194286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5c180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10616468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e894fea100","size":5,"soft_fail":false,"target_va":5384814976,"va":5374198503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5dff0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10627403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e84b29a200","size":5,"soft_fail":false,"target_va":5384822768,"va":5374195360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5e310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10627901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e83d2ba200","size":5,"soft_fail":false,"target_va":5384823568,"va":5374195662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f71790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10703425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e84152a300","size":5,"soft_fail":false,"target_va":5384902544,"va":5374199114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28757721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8d9ceb601","size":5,"soft_fail":false,"target_va":5402958528,"va":5374200802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420de280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28972917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e87517ba01","size":5,"soft_fail":false,"target_va":5403173504,"va":5374200582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29399350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e83699c001","size":5,"soft_fail":false,"target_va":5403593344,"va":5374193989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29393286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e88681c001","size":5,"soft_fail":false,"target_va":5403593392,"va":5374200101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28777484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e80c1cb701","size":5,"soft_fail":false,"target_va":5402978144,"va":5374200655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2893543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e819d9d3ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5374198770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28777806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e84e1db701","size":5,"soft_fail":false,"target_va":5402978688,"va":5374200877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28778296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8381fb701","size":5,"soft_fail":false,"target_va":5402978912,"va":5374200611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28778247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8071fb701","size":5,"soft_fail":false,"target_va":5402978880,"va":5374200628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28778658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8a220b701","size":5,"soft_fail":false,"target_va":5402979152,"va":5374200489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28778812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e83c21b701","size":5,"soft_fail":false,"target_va":5402979344,"va":5374200527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-5623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e809eaffff","size":5,"soft_fail":false,"target_va":5374190272,"va":5374195890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28786434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8023fb701","size":5,"soft_fail":false,"target_va":5402980128,"va":5374193689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e80cf5ffff","size":5,"soft_fail":false,"target_va":5374190464,"va":5374193263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28787318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e87642b701","size":5,"soft_fail":false,"target_va":5402980656,"va":5374193333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t28774930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e81212b701","size":5,"soft_fail":false,"target_va":5402968992,"va":5374194057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t28771017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8c902b701","size":5,"soft_fail":false,"target_va":5402970800,"va":5374199778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2163051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e86b012100","size":5,"soft_fail":false,"target_va":5376356912,"va":5374193856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2166938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e89a102100","size":5,"soft_fail":false,"target_va":5376360560,"va":5374193617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2162906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8da002100","size":5,"soft_fail":false,"target_va":5376356144,"va":5374193233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rapier3d::geometry::collider::Collider::compute_aabb
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10191576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e8d8829b00","size":5,"soft_fail":false,"target_va":5384386976,"va":5374195395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3171209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374193104,"raw_hex":"e889633000","size":5,"soft_fail":false,"target_va":5377366224,"va":5374195010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374193104,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5374194009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21412,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":11287,"source_line":1126,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21204,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":27988,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":18804,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":23760,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":21056,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":17508,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":28248,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":397,"record_offset":17784,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":397,"record_offset":10056,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":397,"record_offset":11556,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":22400,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":19900,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":10356,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":26196,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":23896,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":17620,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":397,"record_offset":10116,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":397,"record_offset":17876,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":14712,"source_line":117,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":397,"record_offset":18384,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":397,"record_offset":18832,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":11612,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22738,"source_line":317,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::remove_invalid_polylines
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":17756,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22751,"source_line":493,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::ColliderDiff::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":27116,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22764,"source_line":196,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::as_ivec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":23700,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22757,"source_line":937,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":23364,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22216,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":23236,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22428,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22792,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22616,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22736,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":23148,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":397,"record_offset":21656,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":397,"record_offset":21936,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":397,"record_offset":10392,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":397,"record_offset":10564,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::add_node
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.6.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/petgraph-0.6.3/src/graphmap.rs
//     evidence={"module":397,"record_offset":24724,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22760,"source_line":247,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::all_edges
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.6.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/petgraph-0.6.3/src/graphmap.rs
//     evidence={"module":397,"record_offset":28072,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22765,"source_line":480,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":8988,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":7400,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":7976,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":10620,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22727,"source_line":1450,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":11036,"parent_record_offset":7064,"ancestor_record_offset":7064,"inlinee":22730,"source_line":1499,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":18000,"parent_record_offset":17976,"ancestor_record_offset":7064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":7064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":16780,"parent_record_offset":16752,"ancestor_record_offset":7064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":7064,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":17976,"parent_record_offset":17944,"ancestor_record_offset":7064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":16752,"parent_record_offset":16536,"ancestor_record_offset":7064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":397,"record_offset":17916,"parent_record_offset":17876,"ancestor_record_offset":7064,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21444,"parent_record_offset":21412,"ancestor_record_offset":7064,"inlinee":11288,"source_line":1182,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21232,"parent_record_offset":21204,"ancestor_record_offset":7064,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21472,"parent_record_offset":21444,"ancestor_record_offset":7064,"inlinee":11289,"source_line":1270,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21264,"parent_record_offset":21232,"ancestor_record_offset":7064,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21288,"parent_record_offset":21264,"ancestor_record_offset":7064,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::iter::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":397,"record_offset":21712,"parent_record_offset":21684,"ancestor_record_offset":7064,"inlinee":22754,"source_line":44,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":397,"record_offset":21312,"parent_record_offset":21288,"ancestor_record_offset":7064,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":7064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":23832,"parent_record_offset":23800,"ancestor_record_offset":7064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":7064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":397,"record_offset":18100,"parent_record_offset":17876,"ancestor_record_offset":7064,"inlinee":11196,"source_line":1613,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":397,"record_offset":18152,"parent_record_offset":18100,"ancestor_record_offset":7064,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":23800,"parent_record_offset":23760,"ancestor_record_offset":7064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":7064,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":24968,"parent_record_offset":24824,"ancestor_record_offset":7064,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":26356,"parent_record_offset":26328,"ancestor_record_offset":7064,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":14372,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":397,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":7064,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":397,"record_offset":24504,"parent_record_offset":24480,"ancestor_record_offset":7064,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":397,"record_offset":10084,"parent_record_offset":10056,"ancestor_record_offset":7064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":397,"record_offset":15836,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9408,"parent_record_offset":9380,"ancestor_record_offset":7064,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":14728,"parent_record_offset":14700,"ancestor_record_offset":7064,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9460,"parent_record_offset":9436,"ancestor_record_offset":7064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":7064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":7064,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9380,"parent_record_offset":9348,"ancestor_record_offset":7064,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9552,"parent_record_offset":9380,"ancestor_record_offset":7064,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":9664,"parent_record_offset":9640,"ancestor_record_offset":7064,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":397,"record_offset":10184,"parent_record_offset":10144,"ancestor_record_offset":7064,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":397,"record_offset":15928,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":7064,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":7064,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":10940,"parent_record_offset":10908,"ancestor_record_offset":7064,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":7064,"inlinee":8944,"source_line":21,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":397,"record_offset":18208,"parent_record_offset":18100,"ancestor_record_offset":7064,"inlinee":8943,"source_line":216,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":397,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":7064,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":397,"record_offset":27436,"parent_record_offset":27400,"ancestor_record_offset":7064,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":397,"record_offset":28272,"parent_record_offset":28248,"ancestor_record_offset":7064,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":397,"record_offset":9484,"parent_record_offset":9460,"ancestor_record_offset":7064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":7064,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":18700,"parent_record_offset":18672,"ancestor_record_offset":7064,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":7064,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":7064,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":7064,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":397,"record_offset":7612,"parent_record_offset":7516,"ancestor_record_offset":7064,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":397,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":7064,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":22496,"parent_record_offset":22456,"ancestor_record_offset":7064,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":22524,"parent_record_offset":22456,"ancestor_record_offset":7064,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":22272,"parent_record_offset":22240,"ancestor_record_offset":7064,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":7064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":18612,"parent_record_offset":18384,"ancestor_record_offset":7064,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":21380,"parent_record_offset":21204,"ancestor_record_offset":7064,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":8572,"parent_record_offset":8112,"ancestor_record_offset":7064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":9288,"parent_record_offset":9108,"ancestor_record_offset":7064,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":15892,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":18644,"parent_record_offset":18384,"ancestor_record_offset":7064,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":397,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":7064,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=76
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":9616,"parent_record_offset":9316,"ancestor_record_offset":7064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":7064,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":7064,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":24904,"parent_record_offset":24880,"ancestor_record_offset":7064,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":7064,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":7064,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":7064,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7064,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7064,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":8532,"parent_record_offset":8500,"ancestor_record_offset":7064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":24928,"parent_record_offset":24904,"ancestor_record_offset":7064,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":18052,"parent_record_offset":18024,"ancestor_record_offset":7064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":7064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":397,"record_offset":10016,"parent_record_offset":9068,"ancestor_record_offset":7064,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":397,"record_offset":10500,"parent_record_offset":10416,"ancestor_record_offset":7064,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":397,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":7064,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":397,"record_offset":10268,"parent_record_offset":10240,"ancestor_record_offset":7064,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":397,"record_offset":10296,"parent_record_offset":10268,"ancestor_record_offset":7064,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":7548,"parent_record_offset":7516,"ancestor_record_offset":7064,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":7064,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":7064,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":18584,"parent_record_offset":18384,"ancestor_record_offset":7064,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":24996,"parent_record_offset":24968,"ancestor_record_offset":7064,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":7576,"parent_record_offset":7548,"ancestor_record_offset":7064,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":25024,"parent_record_offset":24996,"ancestor_record_offset":7064,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":7064,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":21864,"parent_record_offset":21840,"ancestor_record_offset":7064,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":7064,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":17556,"parent_record_offset":17532,"ancestor_record_offset":7064,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":17696,"parent_record_offset":17620,"ancestor_record_offset":7064,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":16020,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":8628,"parent_record_offset":8604,"ancestor_record_offset":7064,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":16132,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":8276,"parent_record_offset":8168,"ancestor_record_offset":7064,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":397,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":7064,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":397,"record_offset":8008,"parent_record_offset":7976,"ancestor_record_offset":7064,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":397,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":7064,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":397,"record_offset":8112,"parent_record_offset":8080,"ancestor_record_offset":7064,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7064,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7064,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":7064,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":8604,"parent_record_offset":8080,"ancestor_record_offset":7064,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7712,"parent_record_offset":7680,"ancestor_record_offset":7064,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7872,"parent_record_offset":7712,"ancestor_record_offset":7064,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":7064,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":7064,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7432,"parent_record_offset":7400,"ancestor_record_offset":7064,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7064,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7488,"parent_record_offset":7456,"ancestor_record_offset":7064,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":7456,"parent_record_offset":7432,"ancestor_record_offset":7064,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":7064,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":8416,"parent_record_offset":8168,"ancestor_record_offset":7064,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":8388,"parent_record_offset":8168,"ancestor_record_offset":7064,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":8220,"parent_record_offset":8168,"ancestor_record_offset":7064,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":26012,"parent_record_offset":25980,"ancestor_record_offset":7064,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":10444,"parent_record_offset":10416,"ancestor_record_offset":7064,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":18320,"parent_record_offset":18100,"ancestor_record_offset":7064,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":18264,"parent_record_offset":18100,"ancestor_record_offset":7064,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":7064,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":7064,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":397,"record_offset":18288,"parent_record_offset":18264,"ancestor_record_offset":7064,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":397,"record_offset":10144,"parent_record_offset":10116,"ancestor_record_offset":7064,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":397,"record_offset":18672,"parent_record_offset":18644,"ancestor_record_offset":7064,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":397,"record_offset":25956,"parent_record_offset":25932,"ancestor_record_offset":7064,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":397,"record_offset":19676,"parent_record_offset":19652,"ancestor_record_offset":7064,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":397,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":7064,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":19928,"parent_record_offset":19900,"ancestor_record_offset":7064,"inlinee":22753,"source_line":91,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":26136,"parent_record_offset":26108,"ancestor_record_offset":7064,"inlinee":22762,"source_line":95,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::impl$0::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":7064,"inlinee":22736,"source_line":36,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::polyline::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":14428,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":22740,"source_line":517,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$1::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":7064,"inlinee":22742,"source_line":162,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$82::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":15992,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":22744,"source_line":1064,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22328,"parent_record_offset":22240,"ancestor_record_offset":7064,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22884,"parent_record_offset":22860,"ancestor_record_offset":7064,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22860,"parent_record_offset":22824,"ancestor_record_offset":7064,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22824,"parent_record_offset":22792,"ancestor_record_offset":7064,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22240,"parent_record_offset":22216,"ancestor_record_offset":7064,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":22456,"parent_record_offset":22428,"ancestor_record_offset":7064,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/i32/ivec3.rs
//     evidence={"module":397,"record_offset":23728,"parent_record_offset":23700,"ancestor_record_offset":7064,"inlinee":22759,"source_line":68,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":397,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":7064,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":7064,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":397,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":7064,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":397,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":7064,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":7064,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":7064,"inlinee":11730,"source_line":652,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":397,"record_offset":19604,"parent_record_offset":19384,"ancestor_record_offset":7064,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":11120,"parent_record_offset":11092,"ancestor_record_offset":7064,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$71::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":11092,"parent_record_offset":11064,"ancestor_record_offset":7064,"inlinee":22734,"source_line":4848,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":18948,"parent_record_offset":18916,"ancestor_record_offset":7064,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19516,"parent_record_offset":19492,"ancestor_record_offset":7064,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19492,"parent_record_offset":19384,"ancestor_record_offset":7064,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19432,"parent_record_offset":19408,"ancestor_record_offset":7064,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":11420,"parent_record_offset":11184,"ancestor_record_offset":7064,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19812,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":10824,"parent_record_offset":10764,"ancestor_record_offset":7064,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":11184,"parent_record_offset":11148,"ancestor_record_offset":7064,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19408,"parent_record_offset":19384,"ancestor_record_offset":7064,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":7064,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":7064,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":7064,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":18976,"parent_record_offset":18916,"ancestor_record_offset":7064,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":7064,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16568,"parent_record_offset":16536,"ancestor_record_offset":7064,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27340,"parent_record_offset":27316,"ancestor_record_offset":7064,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27400,"parent_record_offset":27340,"ancestor_record_offset":7064,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":7064,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":7064,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19024,"parent_record_offset":19000,"ancestor_record_offset":7064,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16536,"parent_record_offset":16480,"ancestor_record_offset":7064,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16508,"parent_record_offset":16480,"ancestor_record_offset":7064,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":7064,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":7064,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":7064,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":19292,"parent_record_offset":19264,"ancestor_record_offset":7064,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":19352,"parent_record_offset":19236,"ancestor_record_offset":7064,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":11224,"parent_record_offset":11184,"ancestor_record_offset":7064,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27468,"parent_record_offset":27340,"ancestor_record_offset":7064,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19068,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":7064,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":11148,"parent_record_offset":11120,"ancestor_record_offset":7064,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":27292,"parent_record_offset":27268,"ancestor_record_offset":7064,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":19384,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.12.3/src/raw/mod.rs
//     evidence={"module":397,"record_offset":24880,"parent_record_offset":24856,"ancestor_record_offset":7064,"inlinee":6971,"source_line":338,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":19096,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":7064,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":19180,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":19720,"parent_record_offset":19024,"ancestor_record_offset":7064,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":10908,"parent_record_offset":10852,"ancestor_record_offset":7064,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.12.3/src/scopeguard.rs
//     evidence={"module":397,"record_offset":27268,"parent_record_offset":27244,"ancestor_record_offset":7064,"inlinee":8805,"source_line":71,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::clear
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":397,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":7064,"inlinee":14996,"source_line":431,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":397,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":7064,"inlinee":11729,"source_line":291,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$21::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":397,"record_offset":11064,"parent_record_offset":11036,"ancestor_record_offset":7064,"inlinee":22732,"source_line":1692,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::entry
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map.rs
//     evidence={"module":397,"record_offset":24756,"parent_record_offset":24724,"ancestor_record_offset":7064,"inlinee":8494,"source_line":390,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map.rs
//     evidence={"module":397,"record_offset":28096,"parent_record_offset":28072,"ancestor_record_offset":7064,"inlinee":22766,"source_line":216,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::core::Entry::or_insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map/core.rs
//     evidence={"module":397,"record_offset":24788,"parent_record_offset":24724,"ancestor_record_offset":7064,"inlinee":22761,"source_line":504,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::core::impl$2::as_entries
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map/core.rs
//     evidence={"module":397,"record_offset":28152,"parent_record_offset":28120,"ancestor_record_offset":7064,"inlinee":22770,"source_line":104,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::core::raw::OccupiedEntry::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map/core/raw.rs
//     evidence={"module":397,"record_offset":24856,"parent_record_offset":24824,"ancestor_record_offset":7064,"inlinee":6970,"source_line":152,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::core::raw::OccupiedEntry::into_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map/core/raw.rs
//     evidence={"module":397,"record_offset":24824,"parent_record_offset":24788,"ancestor_record_offset":7064,"inlinee":6969,"source_line":159,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::impl$1::as_entries
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map.rs
//     evidence={"module":397,"record_offset":28120,"parent_record_offset":28096,"ancestor_record_offset":7064,"inlinee":22768,"source_line":109,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":397,"record_offset":21684,"parent_record_offset":21656,"ancestor_record_offset":7064,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":397,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":7064,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":397,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":7064,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":397,"record_offset":25980,"parent_record_offset":25956,"ancestor_record_offset":7064,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/partitioning/qbvh/traversal.rs
//     evidence={"module":397,"record_offset":16328,"parent_record_offset":16268,"ancestor_record_offset":7064,"inlinee":22750,"source_line":31,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_visitor.rs
//     evidence={"module":397,"record_offset":16296,"parent_record_offset":16268,"ancestor_record_offset":7064,"inlinee":22748,"source_line":20,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":13324,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":9028,"parent_record_offset":8988,"ancestor_record_offset":7064,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":11736,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":9152,"parent_record_offset":9108,"ancestor_record_offset":7064,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":9316,"parent_record_offset":9108,"ancestor_record_offset":7064,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":9588,"parent_record_offset":9316,"ancestor_record_offset":7064,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":12312,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::pipeline::query_pipeline::QueryPipeline::colliders_with_aabb_intersecting_aabb
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/pipeline/query_pipeline/mod.rs
//     evidence={"module":397,"record_offset":16268,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":22746,"source_line":581,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":397,"record_offset":18868,"parent_record_offset":18832,"ancestor_record_offset":7064,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":397,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":7064,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":27144,"parent_record_offset":27116,"ancestor_record_offset":7064,"inlinee":14995,"source_line":343,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":7064,"inlinee":22728,"source_line":191,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":14924,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":22727,"source_line":1450,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$19::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":15340,"parent_record_offset":11612,"ancestor_record_offset":7064,"inlinee":22730,"source_line":1499,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":22360,"parent_record_offset":22328,"ancestor_record_offset":7064,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":7064,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":14580,"parent_record_offset":14552,"ancestor_record_offset":7064,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14700,"parent_record_offset":14552,"ancestor_record_offset":7064,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":7064,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":9180,"parent_record_offset":9152,"ancestor_record_offset":7064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":397,"record_offset":9212,"parent_record_offset":9180,"ancestor_record_offset":7064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":9108,"parent_record_offset":9068,"ancestor_record_offset":7064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":9068,"parent_record_offset":9028,"ancestor_record_offset":7064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":19868,"parent_record_offset":18832,"ancestor_record_offset":7064,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58E1
// 首条位置: {"module":397,"inlinee":22753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":397,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":19928,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58EA
// 首条位置: {"module":397,"inlinee":22762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":397,"record_offset":26160,"parent_record_offset":26136,"ancestor_record_offset":26136,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B38
// 首条位置: {"module":406,"inlinee":23352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1308,"record_offset":102068,"parent_record_offset":102044,"ancestor_record_offset":102044,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1308,"record_offset":102116,"parent_record_offset":102092,"ancestor_record_offset":102044,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1308,"record_offset":102092,"parent_record_offset":102068,"ancestor_record_offset":102044,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":397,"record_offset":32200,"rva":5496096,"va":5374205216}
// 编译实例 RVAs: 0x53dd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1456367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e811c7e9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374206074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140566670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e8c3880200","size":5,"soft_fail":false,"target_va":5374371440,"va":5374205352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2897364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e82ccad3ff","size":5,"soft_fail":false,"target_va":5371308608,"va":5374205967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::shape::trimesh::TriMesh::triangle
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10735909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e825d1a300","size":5,"soft_fail":false,"target_va":5384941360,"va":5374205446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::expand
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10215307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e88bdf9b00","size":5,"soft_fail":false,"target_va":5384420960,"va":5374205648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::inclusive_contains_point
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t10215503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e84fe09b00","size":5,"soft_fail":false,"target_va":5384421264,"va":5374205756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::aabb
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8061536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e860027b00","size":5,"soft_fail":false,"target_va":5382267152,"va":5374205611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::intersect_triangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8061822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e87e037b00","size":5,"soft_fail":false,"target_va":5382267392,"va":5374205565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::normal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8061072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374205216,"raw_hex":"e890007b00","size":5,"soft_fail":false,"target_va":5382266976,"va":5374205899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":32388,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":32840,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":32444,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":32972,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":32556,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::from_parry_triangle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":32648,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":22773,"source_line":301,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":32776,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":32720,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_simultaneous_visitor.rs
//     evidence={"module":397,"record_offset":32416,"parent_record_offset":32200,"ancestor_record_offset":32200,"inlinee":22772,"source_line":21,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":33148,"parent_record_offset":33124,"ancestor_record_offset":32200,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":33124,"parent_record_offset":33024,"ancestor_record_offset":32200,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":33056,"parent_record_offset":33024,"ancestor_record_offset":32200,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":32908,"parent_record_offset":32876,"ancestor_record_offset":32200,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":33024,"parent_record_offset":32996,"ancestor_record_offset":32200,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":32876,"parent_record_offset":32840,"ancestor_record_offset":32200,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":33088,"parent_record_offset":33056,"ancestor_record_offset":32200,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":32808,"parent_record_offset":32776,"ancestor_record_offset":32200,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":32996,"parent_record_offset":32972,"ancestor_record_offset":32200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":32516,"parent_record_offset":32492,"ancestor_record_offset":32200,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":32608,"parent_record_offset":32556,"ancestor_record_offset":32200,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":32940,"parent_record_offset":32840,"ancestor_record_offset":32200,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":32468,"parent_record_offset":32444,"ancestor_record_offset":32200,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":32492,"parent_record_offset":32468,"ancestor_record_offset":32200,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BB3
// 首条位置: {"module":409,"inlinee":23475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":6112,"parent_record_offset":6084,"ancestor_record_offset":6084,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":6084,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":6152,"parent_record_offset":6112,"ancestor_record_offset":6084,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":6208,"parent_record_offset":6112,"ancestor_record_offset":6084,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00409.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::from_parry_triangle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58F5
// 首条位置: {"module":397,"inlinee":22773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58D2
// 首条位置: {"module":397,"inlinee":22738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":14896,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":14372,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":397,"record_offset":15836,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":397,"record_offset":15928,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":15892,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":16360,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":16020,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":16132,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":14428,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":22740,"source_line":517,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$82::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":15992,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":22744,"source_line":1064,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":13324,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":11736,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":12312,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::pipeline::query_pipeline::QueryPipeline::colliders_with_aabb_intersecting_aabb
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/pipeline/query_pipeline/mod.rs
//     evidence={"module":397,"record_offset":16268,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":22746,"source_line":581,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":14924,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":22727,"source_line":1450,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":15340,"parent_record_offset":11612,"ancestor_record_offset":11612,"inlinee":22730,"source_line":1499,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":16780,"parent_record_offset":16752,"ancestor_record_offset":11612,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":16752,"parent_record_offset":16536,"ancestor_record_offset":11612,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":11612,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":11612,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":11612,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13724,"parent_record_offset":13696,"ancestor_record_offset":11612,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":14728,"parent_record_offset":14700,"ancestor_record_offset":11612,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":11612,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13752,"parent_record_offset":13724,"ancestor_record_offset":11612,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13664,"parent_record_offset":13636,"ancestor_record_offset":11612,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13696,"parent_record_offset":13664,"ancestor_record_offset":11612,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13868,"parent_record_offset":13696,"ancestor_record_offset":11612,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":11612,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":11612,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":397,"record_offset":15244,"parent_record_offset":15212,"ancestor_record_offset":11612,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":397,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":11612,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":11612,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":11612,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":11612,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":397,"record_offset":11948,"parent_record_offset":11852,"ancestor_record_offset":11612,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":11612,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":12908,"parent_record_offset":12448,"ancestor_record_offset":11612,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":13608,"parent_record_offset":13432,"ancestor_record_offset":11612,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":397,"record_offset":12260,"parent_record_offset":12232,"ancestor_record_offset":11612,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":13932,"parent_record_offset":13636,"ancestor_record_offset":11612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":11612,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":11612,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":11612,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":11612,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":12660,"parent_record_offset":12636,"ancestor_record_offset":11612,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":12868,"parent_record_offset":12836,"ancestor_record_offset":11612,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":11612,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":397,"record_offset":14332,"parent_record_offset":13396,"ancestor_record_offset":11612,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":11884,"parent_record_offset":11852,"ancestor_record_offset":11612,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":11612,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":11912,"parent_record_offset":11884,"ancestor_record_offset":11612,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":11612,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":11612,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":11612,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":12612,"parent_record_offset":12504,"ancestor_record_offset":11612,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":397,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":11612,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":397,"record_offset":12344,"parent_record_offset":12312,"ancestor_record_offset":11612,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":397,"record_offset":12476,"parent_record_offset":12448,"ancestor_record_offset":11612,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":397,"record_offset":12448,"parent_record_offset":12416,"ancestor_record_offset":11612,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11612,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":11612,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":11612,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":397,"record_offset":12940,"parent_record_offset":12416,"ancestor_record_offset":11612,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":12048,"parent_record_offset":12016,"ancestor_record_offset":11612,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":12208,"parent_record_offset":12048,"ancestor_record_offset":11612,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11612,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":11612,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":12232,"parent_record_offset":12208,"ancestor_record_offset":11612,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":11824,"parent_record_offset":11792,"ancestor_record_offset":11612,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":397,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11612,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":12504,"parent_record_offset":12476,"ancestor_record_offset":11612,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":12752,"parent_record_offset":12504,"ancestor_record_offset":11612,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":12724,"parent_record_offset":12504,"ancestor_record_offset":11612,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":397,"record_offset":12556,"parent_record_offset":12504,"ancestor_record_offset":11612,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":397,"record_offset":15956,"parent_record_offset":15928,"ancestor_record_offset":11612,"inlinee":22736,"source_line":36,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$1::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":397,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":11612,"inlinee":22742,"source_line":162,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":11612,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":14996,"parent_record_offset":14972,"ancestor_record_offset":11612,"inlinee":11730,"source_line":652,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":11612,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$71::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":397,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":11612,"inlinee":22734,"source_line":4848,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15700,"parent_record_offset":15464,"ancestor_record_offset":11612,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15128,"parent_record_offset":15068,"ancestor_record_offset":11612,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15464,"parent_record_offset":15436,"ancestor_record_offset":11612,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":11612,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16568,"parent_record_offset":16536,"ancestor_record_offset":11612,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15100,"parent_record_offset":15068,"ancestor_record_offset":11612,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":11612,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16536,"parent_record_offset":16480,"ancestor_record_offset":11612,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16508,"parent_record_offset":16480,"ancestor_record_offset":11612,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":11612,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":11612,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":15532,"parent_record_offset":15504,"ancestor_record_offset":11612,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":397,"record_offset":15504,"parent_record_offset":15464,"ancestor_record_offset":11612,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":11612,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":397,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":11612,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":15156,"parent_record_offset":15128,"ancestor_record_offset":11612,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":397,"record_offset":15212,"parent_record_offset":15156,"ancestor_record_offset":11612,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":397,"record_offset":14972,"parent_record_offset":14948,"ancestor_record_offset":11612,"inlinee":11729,"source_line":291,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$21::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":397,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":11612,"inlinee":22732,"source_line":1692,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/partitioning/qbvh/traversal.rs
//     evidence={"module":397,"record_offset":16328,"parent_record_offset":16268,"ancestor_record_offset":11612,"inlinee":22750,"source_line":31,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_visitor.rs
//     evidence={"module":397,"record_offset":16296,"parent_record_offset":16268,"ancestor_record_offset":11612,"inlinee":22748,"source_line":20,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":13360,"parent_record_offset":13324,"ancestor_record_offset":11612,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":13472,"parent_record_offset":13432,"ancestor_record_offset":11612,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":13636,"parent_record_offset":13432,"ancestor_record_offset":11612,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":397,"record_offset":13904,"parent_record_offset":13636,"ancestor_record_offset":11612,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":397,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":11612,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":397,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":11612,"inlinee":22728,"source_line":191,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":11612,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":14580,"parent_record_offset":14552,"ancestor_record_offset":11612,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14700,"parent_record_offset":14552,"ancestor_record_offset":11612,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":11612,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":13500,"parent_record_offset":13472,"ancestor_record_offset":11612,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":397,"record_offset":13532,"parent_record_offset":13500,"ancestor_record_offset":11612,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":13432,"parent_record_offset":13396,"ancestor_record_offset":11612,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":13396,"parent_record_offset":13360,"ancestor_record_offset":11612,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 355
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B23
// 首条位置: {"module":406,"inlinee":23331}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":406,"record_offset":64412,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":406,"record_offset":64468,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":406,"record_offset":64440,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":406,"record_offset":65000,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":62988,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":406,"record_offset":64320,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":406,"record_offset":65136,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":64720,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":22782,"source_line":1178,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":64496,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":22775,"source_line":210,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::SubworldColliderHandle::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":62856,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":23332,"source_line":184,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$45::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":62884,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":23334,"source_line":173,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider::Collider::shape
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/geometry/collider.rs
//     evidence={"module":406,"record_offset":64356,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":23347,"source_line":431,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":406,"record_offset":64524,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":22777,"source_line":863,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":406,"record_offset":63212,"parent_record_offset":62704,"ancestor_record_offset":62704,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":406,"record_offset":65312,"parent_record_offset":65288,"ancestor_record_offset":62704,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":406,"record_offset":65288,"parent_record_offset":65188,"ancestor_record_offset":62704,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":406,"record_offset":65220,"parent_record_offset":65188,"ancestor_record_offset":62704,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":406,"record_offset":65072,"parent_record_offset":65040,"ancestor_record_offset":62704,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":406,"record_offset":65188,"parent_record_offset":65160,"ancestor_record_offset":62704,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":406,"record_offset":65040,"parent_record_offset":65000,"ancestor_record_offset":62704,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$56::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63060,"parent_record_offset":63036,"ancestor_record_offset":62704,"inlinee":23343,"source_line":1549,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$57::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63084,"parent_record_offset":63060,"ancestor_record_offset":62704,"inlinee":23345,"source_line":1565,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63140,"parent_record_offset":63116,"ancestor_record_offset":62704,"inlinee":12578,"source_line":1549,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":63164,"parent_record_offset":63140,"ancestor_record_offset":62704,"inlinee":12580,"source_line":1565,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":64132,"parent_record_offset":64104,"ancestor_record_offset":62704,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":406,"record_offset":63492,"parent_record_offset":63468,"ancestor_record_offset":62704,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":406,"record_offset":63576,"parent_record_offset":63552,"ancestor_record_offset":62704,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":406,"record_offset":63516,"parent_record_offset":63492,"ancestor_record_offset":62704,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":406,"record_offset":65252,"parent_record_offset":65220,"ancestor_record_offset":62704,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":406,"record_offset":63692,"parent_record_offset":63668,"ancestor_record_offset":62704,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":406,"record_offset":64652,"parent_record_offset":64556,"ancestor_record_offset":62704,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":406,"record_offset":65160,"parent_record_offset":65136,"ancestor_record_offset":62704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":406,"record_offset":64036,"parent_record_offset":64012,"ancestor_record_offset":62704,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":406,"record_offset":63928,"parent_record_offset":63904,"ancestor_record_offset":62704,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":406,"record_offset":65104,"parent_record_offset":65000,"ancestor_record_offset":62704,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":406,"record_offset":64588,"parent_record_offset":64556,"ancestor_record_offset":62704,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":406,"record_offset":64616,"parent_record_offset":64588,"ancestor_record_offset":62704,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$15::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":406,"record_offset":63036,"parent_record_offset":63012,"ancestor_record_offset":62704,"inlinee":23342,"source_line":72,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::impl$7::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":406,"record_offset":63012,"parent_record_offset":62988,"ancestor_record_offset":62704,"inlinee":23340,"source_line":29,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$45::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":64160,"parent_record_offset":64132,"ancestor_record_offset":62704,"inlinee":23334,"source_line":173,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$82::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":406,"record_offset":62908,"parent_record_offset":62884,"ancestor_record_offset":62704,"inlinee":22744,"source_line":1064,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":406,"record_offset":64104,"parent_record_offset":64076,"ancestor_record_offset":62704,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":406,"record_offset":63260,"parent_record_offset":63236,"ancestor_record_offset":62704,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":406,"record_offset":63284,"parent_record_offset":63260,"ancestor_record_offset":62704,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":406,"record_offset":64076,"parent_record_offset":63856,"ancestor_record_offset":62704,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":406,"record_offset":63316,"parent_record_offset":63284,"ancestor_record_offset":62704,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63988,"parent_record_offset":63964,"ancestor_record_offset":62704,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63964,"parent_record_offset":63856,"ancestor_record_offset":62704,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63904,"parent_record_offset":63880,"ancestor_record_offset":62704,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63824,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63880,"parent_record_offset":63856,"ancestor_record_offset":62704,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63368,"parent_record_offset":63344,"ancestor_record_offset":62704,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63344,"parent_record_offset":63284,"ancestor_record_offset":62704,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63392,"parent_record_offset":63368,"ancestor_record_offset":62704,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":63636,"parent_record_offset":63608,"ancestor_record_offset":62704,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":63668,"parent_record_offset":63636,"ancestor_record_offset":62704,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":63608,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63440,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":63856,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":63468,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":63552,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":63732,"parent_record_offset":63392,"ancestor_record_offset":62704,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":406,"record_offset":63236,"parent_record_offset":63212,"ancestor_record_offset":62704,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::impl$33::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":406,"record_offset":62956,"parent_record_offset":62932,"ancestor_record_offset":62704,"inlinee":23338,"source_line":48,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::geometry::collider_components::impl$27::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/geometry/collider_components.rs
//     evidence={"module":406,"record_offset":62932,"parent_record_offset":62884,"ancestor_record_offset":62704,"inlinee":23336,"source_line":9,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":406,"record_offset":64556,"parent_record_offset":64524,"ancestor_record_offset":62704,"inlinee":22778,"source_line":538,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":406,"record_offset":64680,"parent_record_offset":64652,"ancestor_record_offset":62704,"inlinee":22780,"source_line":542,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::remove_invalid_polylines
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58DF
// 首条位置: {"module":397,"inlinee":22751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 494
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFF4
// 首条位置: {"module":1308,"inlinee":61428}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":40972,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41000,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41224,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":27427,"source_line":2818,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter_mut::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":1308,"record_offset":41812,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":20374,"source_line":36,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41592,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41972,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41504,"parent_record_offset":40908,"ancestor_record_offset":40908,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41288,"parent_record_offset":41256,"ancestor_record_offset":40908,"inlinee":11897,"source_line":1216,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41028,"parent_record_offset":41000,"ancestor_record_offset":40908,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41256,"parent_record_offset":41224,"ancestor_record_offset":40908,"inlinee":20372,"source_line":1149,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41064,"parent_record_offset":41028,"ancestor_record_offset":40908,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41316,"parent_record_offset":41288,"ancestor_record_offset":40908,"inlinee":11289,"source_line":1270,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41132,"parent_record_offset":41028,"ancestor_record_offset":40908,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41156,"parent_record_offset":41132,"ancestor_record_offset":40908,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":41180,"parent_record_offset":41156,"ancestor_record_offset":40908,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":41100,"parent_record_offset":41064,"ancestor_record_offset":40908,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":41532,"parent_record_offset":41504,"ancestor_record_offset":40908,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":42000,"parent_record_offset":41972,"ancestor_record_offset":40908,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1308,"record_offset":41404,"parent_record_offset":41380,"ancestor_record_offset":40908,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1308,"record_offset":41872,"parent_record_offset":41840,"ancestor_record_offset":40908,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1308,"record_offset":41356,"parent_record_offset":41256,"ancestor_record_offset":40908,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1308,"record_offset":41380,"parent_record_offset":41356,"ancestor_record_offset":40908,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1308,"record_offset":41840,"parent_record_offset":41812,"ancestor_record_offset":40908,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41728,"parent_record_offset":41692,"ancestor_record_offset":40908,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41692,"parent_record_offset":41664,"ancestor_record_offset":40908,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41664,"parent_record_offset":41624,"ancestor_record_offset":40908,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":41624,"parent_record_offset":41592,"ancestor_record_offset":40908,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":41764,"parent_record_offset":41728,"ancestor_record_offset":40908,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 517
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x58D4
// 首条位置: {"module":397,"inlinee":22740}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":397,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":14728,"parent_record_offset":14700,"ancestor_record_offset":14428,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":14428,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":397,"record_offset":14756,"parent_record_offset":14700,"ancestor_record_offset":14428,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":397,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":14428,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":397,"record_offset":14656,"parent_record_offset":14632,"ancestor_record_offset":14428,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":14428,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14428,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":14580,"parent_record_offset":14552,"ancestor_record_offset":14428,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14700,"parent_record_offset":14552,"ancestor_record_offset":14428,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":397,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":14428,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":397,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14428,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":397,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":14428,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":14552,"parent_record_offset":14528,"ancestor_record_offset":14428,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":397,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":14428,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines
// 记录形式: compiled_procedure; instances=1
// 源码行: 523
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1397,"record_offset":6976,"rva":16968256,"va":5385677376}
// 编译实例 RVAs: 0x102ea40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12932366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8f2aa3aff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385682073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d83a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2800329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e83745d5ff","size":5,"soft_fail":false,"target_va":5382879872,"va":5385680196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2369722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e846d7dbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385677861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420daa70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17480905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8c9bc0a01","size":5,"soft_fail":false,"target_va":5403159152,"va":5385678242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17915228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e85c5d1101","size":5,"soft_fail":false,"target_va":5403593392,"va":5385678159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow::hfebc33fb49dcc2d1 (.llvm.3478840621844800694)
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t168101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8a5900200","size":5,"soft_fail":false,"target_va":5385846944,"va":5385678838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17240190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e87e100701","size":5,"soft_fail":false,"target_va":5402918528,"va":5385678333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14372975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e891af24ff","size":5,"soft_fail":false,"target_va":5371307280,"va":5385680250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17295243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e88be70701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385683440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t17296008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e888ea0701","size":5,"soft_fail":false,"target_va":5402978880,"va":5385682867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=23
//     evidence={"assembly":"\tcallq\t17297034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e88aee0701","size":5,"soft_fail":false,"target_va":5402979344,"va":5385682305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t17295946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e84aea0701","size":5,"soft_fail":false,"target_va":5402978944,"va":5385682993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8a8daffff","size":5,"soft_fail":false,"target_va":5385675168,"va":5385684723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17302213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8c5020801","size":5,"soft_fail":false,"target_va":5402980128,"va":5385677910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e8a8f7ffff","size":5,"soft_fail":false,"target_va":5385675360,"va":5385677491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17303083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e82b060801","size":5,"soft_fail":false,"target_va":5402980656,"va":5385677568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e864180000","size":5,"soft_fail":false,"target_va":5385684784,"va":5385678535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9321170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e82ec571ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385678077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9317286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e85ad471ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385677841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9321333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385677376,"raw_hex":"e88bc471ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385677472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-7217","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385677376,"raw_hex":"e9cfe3ffff","size":5,"soft_fail":false,"target_va":5385675168,"va":5385682380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11840,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":24156,"source_line":1565,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::back_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":17008,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":21856,"source_line":1589,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11608,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":12792,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":12200,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_front
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":13188,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7709,"source_line":1658,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1397,"record_offset":16152,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":10715,"source_line":46,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10388,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":16600,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10464,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":11132,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10916,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1397,"record_offset":11352,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1397,"record_offset":24576,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1397,"record_offset":21308,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1397,"record_offset":16368,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1397,"record_offset":14628,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1397,"record_offset":24436,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1397,"record_offset":23160,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1397,"record_offset":10804,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1397,"record_offset":11012,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":1397,"record_offset":11104,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":64937,"source_line":813,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":1397,"record_offset":11472,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":64939,"source_line":818,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":1397,"record_offset":11076,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":64935,"source_line":809,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":12024,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":12112,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":8860,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":7272,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":7848,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1397,"record_offset":9908,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1397,"record_offset":10268,"parent_record_offset":6976,"ancestor_record_offset":6976,"inlinee":17667,"source_line":757,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1397,"record_offset":16928,"parent_record_offset":16904,"ancestor_record_offset":6976,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1397,"record_offset":16904,"parent_record_offset":16812,"ancestor_record_offset":6976,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":16304,"parent_record_offset":16176,"ancestor_record_offset":6976,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":12260,"parent_record_offset":12200,"ancestor_record_offset":6976,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":6976,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":6976,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":6976,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":12232,"parent_record_offset":12200,"ancestor_record_offset":6976,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":21364,"parent_record_offset":21336,"ancestor_record_offset":6976,"inlinee":24159,"source_line":1636,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":16176,"parent_record_offset":16152,"ancestor_record_offset":6976,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11764,"parent_record_offset":11632,"ancestor_record_offset":6976,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11668,"parent_record_offset":11632,"ancestor_record_offset":6976,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11696,"parent_record_offset":11668,"ancestor_record_offset":6976,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":13420,"parent_record_offset":13188,"ancestor_record_offset":6976,"inlinee":7713,"source_line":207,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":16392,"parent_record_offset":16368,"ancestor_record_offset":6976,"inlinee":23770,"source_line":2786,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":16500,"parent_record_offset":16468,"ancestor_record_offset":6976,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$3::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1397,"record_offset":21336,"parent_record_offset":21308,"ancestor_record_offset":6976,"inlinee":64941,"source_line":181,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":1397,"record_offset":16444,"parent_record_offset":16392,"ancestor_record_offset":6976,"inlinee":23772,"source_line":12,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1397,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":6976,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":6976,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":16840,"parent_record_offset":16812,"ancestor_record_offset":6976,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":10584,"parent_record_offset":10556,"ancestor_record_offset":6976,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":11800,"parent_record_offset":11764,"ancestor_record_offset":6976,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":6976,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1397,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":6976,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":16648,"parent_record_offset":16600,"ancestor_record_offset":6976,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10428,"parent_record_offset":10388,"ancestor_record_offset":6976,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10492,"parent_record_offset":10464,"ancestor_record_offset":6976,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":16524,"parent_record_offset":16500,"ancestor_record_offset":6976,"inlinee":23722,"source_line":900,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10620,"parent_record_offset":10492,"ancestor_record_offset":6976,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":6976,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1397,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":6976,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1397,"record_offset":10768,"parent_record_offset":10492,"ancestor_record_offset":6976,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":6976,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":6976,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9288,"parent_record_offset":9260,"ancestor_record_offset":6976,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":6976,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":6976,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9404,"parent_record_offset":9232,"ancestor_record_offset":6976,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1397,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":6976,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1397,"record_offset":10176,"parent_record_offset":10140,"ancestor_record_offset":6976,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1397,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":6976,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1397,"record_offset":10656,"parent_record_offset":10492,"ancestor_record_offset":6976,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1397,"record_offset":16416,"parent_record_offset":16392,"ancestor_record_offset":6976,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1397,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":6976,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1397,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":6976,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1397,"record_offset":16868,"parent_record_offset":16840,"ancestor_record_offset":6976,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1397,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":6976,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1397,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":6976,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1397,"record_offset":7484,"parent_record_offset":7388,"ancestor_record_offset":6976,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1397,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":6976,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1397,"record_offset":12052,"parent_record_offset":12024,"ancestor_record_offset":6976,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1397,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":6976,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1397,"record_offset":8444,"parent_record_offset":7984,"ancestor_record_offset":6976,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1397,"record_offset":9144,"parent_record_offset":8968,"ancestor_record_offset":6976,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1397,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":6976,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1397,"record_offset":9468,"parent_record_offset":9172,"ancestor_record_offset":6976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1397,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":6976,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1397,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":6976,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1397,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":6976,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1397,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":6976,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1397,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":6976,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1397,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":6976,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1397,"record_offset":9868,"parent_record_offset":8932,"ancestor_record_offset":6976,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1397,"record_offset":7420,"parent_record_offset":7388,"ancestor_record_offset":6976,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1397,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":6976,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1397,"record_offset":11160,"parent_record_offset":11132,"ancestor_record_offset":6976,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1397,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":6976,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1397,"record_offset":7448,"parent_record_offset":7420,"ancestor_record_offset":6976,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1397,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":6976,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1397,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":6976,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1397,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":6976,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1397,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":6976,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1397,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":6976,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1397,"record_offset":8148,"parent_record_offset":8040,"ancestor_record_offset":6976,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1397,"record_offset":7388,"parent_record_offset":7360,"ancestor_record_offset":6976,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1397,"record_offset":7880,"parent_record_offset":7848,"ancestor_record_offset":6976,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1397,"record_offset":8012,"parent_record_offset":7984,"ancestor_record_offset":6976,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1397,"record_offset":7984,"parent_record_offset":7952,"ancestor_record_offset":6976,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1397,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":6976,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1397,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":6976,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1397,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":6976,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1397,"record_offset":8476,"parent_record_offset":7952,"ancestor_record_offset":6976,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7584,"parent_record_offset":7552,"ancestor_record_offset":6976,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7744,"parent_record_offset":7584,"ancestor_record_offset":6976,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7552,"parent_record_offset":7528,"ancestor_record_offset":6976,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7304,"parent_record_offset":7272,"ancestor_record_offset":6976,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":6976,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7360,"parent_record_offset":7328,"ancestor_record_offset":6976,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1397,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":6976,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1397,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":6976,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1397,"record_offset":8288,"parent_record_offset":8040,"ancestor_record_offset":6976,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1397,"record_offset":8260,"parent_record_offset":8040,"ancestor_record_offset":6976,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1397,"record_offset":8092,"parent_record_offset":8040,"ancestor_record_offset":6976,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":1397,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":6976,"inlinee":64939,"source_line":818,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":6976,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":10300,"parent_record_offset":10268,"ancestor_record_offset":6976,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":6976,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":6976,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10336,"parent_record_offset":10300,"ancestor_record_offset":6976,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":6976,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10140,"parent_record_offset":10080,"ancestor_record_offset":6976,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10112,"parent_record_offset":10080,"ancestor_record_offset":6976,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10208,"parent_record_offset":10080,"ancestor_record_offset":6976,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":6976,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\scopeguard.rs
//     evidence={"module":1397,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":6976,"inlinee":8805,"source_line":69,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":8896,"parent_record_offset":8860,"ancestor_record_offset":6976,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1397,"record_offset":9008,"parent_record_offset":8968,"ancestor_record_offset":6976,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":9172,"parent_record_offset":8968,"ancestor_record_offset":6976,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1397,"record_offset":9440,"parent_record_offset":9172,"ancestor_record_offset":6976,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1397,"record_offset":9036,"parent_record_offset":9008,"ancestor_record_offset":6976,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1397,"record_offset":9068,"parent_record_offset":9036,"ancestor_record_offset":6976,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1397,"record_offset":8968,"parent_record_offset":8932,"ancestor_record_offset":6976,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1397,"record_offset":8932,"parent_record_offset":8896,"ancestor_record_offset":6976,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 540
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1397,"record_offset":26152,"rva":16975664,"va":5385684784}
// 编译实例 RVAs: 0x1030730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140e93790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1691800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385684784,"raw_hex":"e8682fe6ff","size":5,"soft_fail":false,"target_va":5383993232,"va":5385685027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f32d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1040422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385684784,"raw_hex":"e8da1ff0ff","size":5,"soft_fail":false,"target_va":5384645936,"va":5385686353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17909418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385684784,"raw_hex":"e8aa461101","size":5,"soft_fail":false,"target_va":5403594288,"va":5385684865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1397,"record_offset":28040,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1397,"record_offset":28096,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::as_ivec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":26604,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":22757,"source_line":937,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::round
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":26460,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":64942,"source_line":636,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$19::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":27676,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":50855,"source_line":1156,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1397,"record_offset":26372,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/i32/ivec3.rs
//     evidence={"module":1397,"record_offset":29948,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":22759,"source_line":68,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1397,"record_offset":26664,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1397,"record_offset":29976,"parent_record_offset":26152,"ancestor_record_offset":26152,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1397,"record_offset":28256,"parent_record_offset":28140,"ancestor_record_offset":26152,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1397,"record_offset":27468,"parent_record_offset":27444,"ancestor_record_offset":26152,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1397,"record_offset":26920,"parent_record_offset":26896,"ancestor_record_offset":26152,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1397,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":26152,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1397,"record_offset":26944,"parent_record_offset":26920,"ancestor_record_offset":26152,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1397,"record_offset":28140,"parent_record_offset":28096,"ancestor_record_offset":26152,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1397,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":26152,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1397,"record_offset":26400,"parent_record_offset":26372,"ancestor_record_offset":26152,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1397,"record_offset":27708,"parent_record_offset":27676,"ancestor_record_offset":26152,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1397,"record_offset":28204,"parent_record_offset":28140,"ancestor_record_offset":26152,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1397,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":26152,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1397,"record_offset":27356,"parent_record_offset":27332,"ancestor_record_offset":26152,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::round
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1397,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":26152,"inlinee":64943,"source_line":202,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/i32/ivec3.rs
//     evidence={"module":1397,"record_offset":26632,"parent_record_offset":26604,"ancestor_record_offset":26152,"inlinee":22759,"source_line":68,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::impl$81::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/i32/ivec3.rs
//     evidence={"module":1397,"record_offset":27492,"parent_record_offset":27468,"ancestor_record_offset":26152,"inlinee":63428,"source_line":19,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1397,"record_offset":27444,"parent_record_offset":27420,"ancestor_record_offset":26152,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":26688,"parent_record_offset":26664,"ancestor_record_offset":26152,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":26712,"parent_record_offset":26688,"ancestor_record_offset":26152,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":27420,"parent_record_offset":27284,"ancestor_record_offset":26152,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1397,"record_offset":26740,"parent_record_offset":26712,"ancestor_record_offset":26152,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27332,"parent_record_offset":27308,"ancestor_record_offset":26152,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":26152,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27568,"parent_record_offset":27544,"ancestor_record_offset":26152,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27252,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27544,"parent_record_offset":26792,"ancestor_record_offset":26152,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":26792,"parent_record_offset":26768,"ancestor_record_offset":26152,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":26768,"parent_record_offset":26712,"ancestor_record_offset":26152,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":29872,"parent_record_offset":28868,"ancestor_record_offset":26152,"inlinee":17280,"source_line":1571,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":26152,"inlinee":17281,"source_line":1565,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":26820,"parent_record_offset":26792,"ancestor_record_offset":26152,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1397,"record_offset":27064,"parent_record_offset":27036,"ancestor_record_offset":26152,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1397,"record_offset":27096,"parent_record_offset":27064,"ancestor_record_offset":26152,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1397,"record_offset":27036,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":26868,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1397,"record_offset":27284,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1397,"record_offset":26896,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1397,"record_offset":26980,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1397,"record_offset":27160,"parent_record_offset":26820,"ancestor_record_offset":26152,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1397,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":26152,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/01397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 809
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDA7
// 首条位置: {"module":1397,"inlinee":64935}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 813
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDA9
// 首条位置: {"module":1397,"inlinee":64937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 818
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDAB
// 首条位置: {"module":1397,"inlinee":64939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::on_exit_game_state
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 848
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5921
// 首条位置: {"module":406,"inlinee":22817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":406,"record_offset":21796,"parent_record_offset":21772,"ancestor_record_offset":21772,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.6.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/petgraph-0.6.3/src/graphmap.rs
//     evidence={"module":406,"record_offset":21852,"parent_record_offset":21772,"ancestor_record_offset":21772,"inlinee":22783,"source_line":241,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":406,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":21772,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::clear
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"1.9.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/indexmap-1.9.1/src/map.rs
//     evidence={"module":406,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":21772,"inlinee":22784,"source_line":268,"archive":"_pdb/A/modules/00406.jsonl.gz"}
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
// 完整结构及原始类型记录: intersect_meshes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::ColliderKey
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>", "constructor": "tuple$", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "petgraph::graphmap::CompactDirection"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::VacantEntry::insert<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >", "constructor": "indexmap::map::core::VacantEntry::insert", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "constructor": "tuple$", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::ColliderKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::insert_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::insert_full", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::swap_remove_full<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::systems::collision::intersect_meshes::ColliderKey>", "constructor": "indexmap::map::core::IndexMapCore::swap_remove_full", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "country_core::systems::collision::intersect_meshes::ColliderKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::swap_remove_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >", "constructor": "indexmap::map::core::IndexMapCore::swap_remove_full", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::clear<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::clear", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::clear<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >", "constructor": "indexmap::map::core::IndexMapCore::clear", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::entry<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >", "constructor": "indexmap::map::core::IndexMapCore::entry", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "indexmap::map::IndexMap", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<indexmap::map::IndexMap<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["indexmap::map::IndexMap<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::Bucket", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "core::ptr::drop_in_place", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::impl$19::default<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::impl$19::default", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::GraphMap::remove_node", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::add_edge<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::GraphMap::add_edge", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::AllEdges", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >", "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::systems::collision::BirdNestCandidateLocation", "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::get_hash::closure_env$0<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >", "constructor": "indexmap::map::core::get_hash::closure_env$0", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::inner::RawTable::reserve_rehash<usize,hashbrown::raw::inner::alloc::inner::Global,indexmap::map::core::get_hash::closure_env$0<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> > >", "constructor": "hashbrown::raw::inner::RawTable::reserve_rehash", "arguments": ["usize", "hashbrown::raw::inner::alloc::inner::Global", "indexmap::map::core::get_hash::closure_env$0<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::get_hash::closure_env$0", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::inner::RawTable::reserve_rehash<usize,hashbrown::raw::inner::alloc::inner::Global,indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "hashbrown::raw::inner::RawTable::reserve_rehash", "arguments": ["usize", "hashbrown::raw::inner::alloc::inner::Global", "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::hash<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::intersect_meshes::ColliderKey>", "constructor": "indexmap::map::IndexMap::hash", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::systems::collision::intersect_meshes::ColliderKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::hash<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >", "constructor": "indexmap::map::IndexMap::hash", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "std::hash::random::RandomState", "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::get_mut<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::intersect_meshes::ColliderKey>", "constructor": "indexmap::map::IndexMap::get_mut", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::systems::collision::intersect_meshes::ColliderKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::IntersectionOutput
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::collision::intersect_meshes::IntersectionOutput", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::IntersectionOutput>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::IntersectionOutput"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::systems::collision::intersect_meshes::IntersectionOutput", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::insert_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::insert_full", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::swap_remove_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >", "constructor": "indexmap::map::core::IndexMapCore::swap_remove_full", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::clear<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::clear", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::Bucket", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "core::ptr::drop_in_place", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::impl$19::default<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::impl$19::default", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::GraphMap::remove_node", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::add_edge<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::GraphMap::add_edge", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::AllEdges", "arguments": ["country_core::systems::collision::intersect_meshes::ColliderKey", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >", "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::systems::collision::BirdNestCandidateLocation", "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::get_hash::closure_env$0", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::inner::RawTable::reserve_rehash<usize,hashbrown::raw::inner::alloc::inner::Global,indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "hashbrown::raw::inner::RawTable::reserve_rehash", "arguments": ["usize", "hashbrown::raw::inner::alloc::inner::Global", "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::hash<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >", "constructor": "indexmap::map::IndexMap::hash", "arguments": ["tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "std::hash::random::RandomState", "tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::MeshIntersectionGraph
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 59
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$10::into_configs<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>", "constructor": "bevy_ecs::schedule::config::impl$10::into_configs", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >", "bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>", "tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::Polyline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::Polyline", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>", "constructor": "alloc::vec::Vec::retain_mut", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::Polyline", "alloc::alloc::Global", "country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::PolylinePoint
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::VertexId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>", "constructor": "tuple$", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: intersect_meshes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: intersect_meshes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 213; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections
// 信息 ID: 8b153fac8f3a90b46d1ee556
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7cf79512bce37b1d09630f9a: void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> @ 0x55eb74 -> 0x53add0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > @ 0xe3b626 -> 0x53add0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
// 信息 ID: 4f50c49a9cefb3abe9b72a24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=19928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
// 信息 ID: 50082b000a37741415939b57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=26136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$2
// 信息 ID: 9b44fd2fa0fe9b1d5120c76d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>,alloc::alloc::Global>; module=1308, inline_record=102044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::from_parry_triangle
// 信息 ID: 589b4c3e07b216c61f613693
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh; module=397, inline_record=32648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh; module=397, inline_record=32684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$0::from
// 信息 ID: 1540aa6931463e3df36b24f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=11580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=15956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$12::default
// 信息 ID: 2fe4b92e561aebe8c79d220b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::impl$0::build; module=395, inline_record=67192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$5::eq
// 信息 ID: 9708a8aef69b66a587782636
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::insert_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>; module=448, inline_record=11964; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::insert_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>; module=448, inline_record=12084; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::swap_remove_full<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::systems::collision::intersect_meshes::ColliderKey>; module=448, inline_record=16136; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::swap_remove_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >; module=448, inline_record=19768; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::swap_remove_full<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >; module=448, inline_record=19888; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::core::IndexMapCore::entry<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global> >; module=448, inline_record=25256; 不是独立调用边。
// [PDB 内联宿主] petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >; module=1268, inline_record=6500; 不是独立调用边。
// [PDB 内联宿主] petgraph::graphmap::GraphMap::add_edge<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >; module=1268, inline_record=11088; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$7::partial_cmp
// 信息 ID: a6c2a3ba89b47a706e20e2a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0>; module=406, inline_record=63012; 不是独立调用边。
// [PDB 内联宿主] petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >; module=1268, inline_record=5824; 不是独立调用边。
// [PDB 内联宿主] petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >; module=1268, inline_record=6104; 不是独立调用边。
// [PDB 内联宿主] petgraph::graphmap::GraphMap::add_edge<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >; module=1268, inline_record=10020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$8::hash
// 信息 ID: b389fe210d811fec15d46e7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] indexmap::map::IndexMap::hash<country_core::systems::collision::intersect_meshes::ColliderKey,alloc::vec::Vec<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::intersect_meshes::ColliderKey>; module=1378, inline_record=12424; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::IndexMap::hash<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >; module=1378, inline_record=14444; 不是独立调用边。
// [PDB 内联宿主] indexmap::map::IndexMap::hash<tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::intersect_meshes::ColliderKey,country_core::systems::collision::intersect_meshes::ColliderKey> >; module=1378, inline_record=14760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh
// 信息 ID: ef6cef24b6e6da211065ea17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x560221 -> 0x53dd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure$0
// 信息 ID: 674d05354818a638182ef149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::intersect_mesh_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0 <- parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_bvtt<u32,u32,parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor<u32,u32,country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_bvtt<u32,u32,parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor<u32,u32,country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0> >; module=409, inline_record=6084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
// 信息 ID: 5ddbde54415574f307048eb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=11612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure$0
// 信息 ID: 3ce30df130bb77baa502643b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0 <- parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0>; module=406, inline_record=62704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::on_exit_game_state
// 信息 ID: 92ce8ed3b4e0daa9f85a3097
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4d0dfad491692889285b6ba6: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>; module=406, inline_record=21772; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >; module=1325, inline_record=2557624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
// 信息 ID: c614e52b97402cc1a499b343
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11104; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
// 信息 ID: 7286c3a9edbbdf9abf4a7bfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11436; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$2::default
// 信息 ID: fee58faf4bf6acb8ab9900fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=14428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
// 信息 ID: 722b62564c9106f4e1d9c5f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=11076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=12764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=13696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=15004; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=15032; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=16980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=17936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=1397, inline_record=18892; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines
// 信息 ID: c680377b231b253d13577b7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x560252 -> 0x102ea40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
// 信息 ID: b126396d42c9dfa0fc226242
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines @ 0x102eec7 -> 0x1030730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines @ 0x102eedc -> 0x1030730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::remove_invalid_polylines
// 信息 ID: 88479fd1d8cbeb06c5e6f5b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=17756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure$0
// 信息 ID: e4846f789245c0df2a3627ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::remove_invalid_polylines", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0 <- alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; module=1308, inline_record=40908; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; module=1308, inline_record=42524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::should_intersect_collider_meshes
// 信息 ID: 6ce851954830af7d656c11de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x560198 -> 0x53e140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
