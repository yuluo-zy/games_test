// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_mesh_atlases.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fbde334650f6842b87e982968c09ae84fdab012a8221edb8e6ea7fa16ec15369
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_mesh_atlases.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1302, 1308, 1321, 1373, 1386
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 27, 35, 46, 52, 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7FA
// 首条位置: {"module":1373,"inlinee":63482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":40004,"parent_record_offset":39980,"ancestor_record_offset":39980,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":48096,"parent_record_offset":48072,"ancestor_record_offset":48072,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 27, 35, 46, 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7FD
// 首条位置: {"module":1373,"inlinee":63485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1373,"record_offset":39304,"parent_record_offset":39276,"ancestor_record_offset":39276,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1373,"record_offset":39328,"parent_record_offset":39304,"ancestor_record_offset":39276,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39352,"parent_record_offset":39328,"ancestor_record_offset":39276,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39376,"parent_record_offset":39352,"ancestor_record_offset":39276,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 27, 35, 46, 52, 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7F5
// 首条位置: {"module":1373,"inlinee":63477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1373,"record_offset":39328,"parent_record_offset":39304,"ancestor_record_offset":39136,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":3968,"parent_record_offset":3932,"ancestor_record_offset":3904,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":4032,"parent_record_offset":3932,"ancestor_record_offset":3904,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39352,"parent_record_offset":39328,"ancestor_record_offset":39136,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39376,"parent_record_offset":39352,"ancestor_record_offset":39136,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1373,"record_offset":39304,"parent_record_offset":39276,"ancestor_record_offset":39136,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1373,"record_offset":39276,"parent_record_offset":39248,"ancestor_record_offset":39136,"inlinee":63485,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":4004,"parent_record_offset":3932,"ancestor_record_offset":3904,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 27, 35, 46, 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7FC
// 首条位置: {"module":1373,"inlinee":63484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1373,"record_offset":39684,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1373,"record_offset":40220,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":39592,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":40588,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1373,"record_offset":49436,"parent_record_offset":45808,"ancestor_record_offset":45808,"inlinee":63487,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":38424,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":39628,"parent_record_offset":38380,"ancestor_record_offset":38380,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1373,"record_offset":40372,"parent_record_offset":40348,"ancestor_record_offset":38380,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1373,"record_offset":40348,"parent_record_offset":40292,"ancestor_record_offset":38380,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1373,"record_offset":40320,"parent_record_offset":40292,"ancestor_record_offset":38380,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1373,"record_offset":40292,"parent_record_offset":40268,"ancestor_record_offset":38380,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1373,"record_offset":48896,"parent_record_offset":48872,"ancestor_record_offset":45808,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1373,"record_offset":39328,"parent_record_offset":39304,"ancestor_record_offset":38380,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":38628,"parent_record_offset":38460,"ancestor_record_offset":38380,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":38668,"parent_record_offset":38628,"ancestor_record_offset":38380,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":38804,"parent_record_offset":38460,"ancestor_record_offset":38380,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1373,"record_offset":49288,"parent_record_offset":49260,"ancestor_record_offset":45808,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1373,"record_offset":40244,"parent_record_offset":40220,"ancestor_record_offset":38380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":38588,"parent_record_offset":38460,"ancestor_record_offset":38380,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39352,"parent_record_offset":39328,"ancestor_record_offset":38380,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1373,"record_offset":39376,"parent_record_offset":39352,"ancestor_record_offset":38380,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1373,"record_offset":39304,"parent_record_offset":39276,"ancestor_record_offset":38380,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1373,"record_offset":47536,"parent_record_offset":47512,"ancestor_record_offset":45808,"inlinee":63464,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1373,"record_offset":39980,"parent_record_offset":39956,"ancestor_record_offset":38380,"inlinee":63482,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1373,"record_offset":39276,"parent_record_offset":39248,"ancestor_record_offset":38380,"inlinee":63485,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1373,"record_offset":38772,"parent_record_offset":38748,"ancestor_record_offset":38380,"inlinee":63477,"source_line":35,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":1373,"record_offset":38708,"parent_record_offset":38460,"ancestor_record_offset":38380,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":39168,"parent_record_offset":39136,"ancestor_record_offset":38380,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":38460,"parent_record_offset":38424,"ancestor_record_offset":38380,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1373,"record_offset":39744,"parent_record_offset":39712,"ancestor_record_offset":38380,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":40004,"parent_record_offset":39980,"ancestor_record_offset":38380,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":48668,"parent_record_offset":48644,"ancestor_record_offset":45808,"inlinee":8558,"source_line":188,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1373,"record_offset":48096,"parent_record_offset":48072,"ancestor_record_offset":45808,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":39248,"parent_record_offset":39168,"ancestor_record_offset":38380,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":38748,"parent_record_offset":38460,"ancestor_record_offset":38380,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1373,"record_offset":48644,"parent_record_offset":48620,"ancestor_record_offset":45808,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7FF
// 首条位置: {"module":1373,"inlinee":63487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1386,"record_offset":59460,"rva":16816720,"va":5385525840}
// 编译实例 RVAs: 0x1009a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17435798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385525840,"raw_hex":"e8960c0a01","size":5,"soft_fail":false,"target_va":5402961856,"va":5385526053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1386,"record_offset":59656,"parent_record_offset":59460,"ancestor_record_offset":59460,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1386,"record_offset":59676,"parent_record_offset":59656,"ancestor_record_offset":59460,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1386,"record_offset":59696,"parent_record_offset":59676,"ancestor_record_offset":59460,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1386,"record_offset":59716,"parent_record_offset":59696,"ancestor_record_offset":59460,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7FB
// 首条位置: {"module":1373,"inlinee":63483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1373,"record_offset":30372,"parent_record_offset":30320,"ancestor_record_offset":30320,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1373,"record_offset":30344,"parent_record_offset":30320,"ancestor_record_offset":30320,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::load_mesh_atlases
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":15776,"rva":13756544,"va":5382465664}
// 编译实例 RVAs: 0xd1e880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9841242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8a6d569ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5382466261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9717175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e849ba6bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382466882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140541720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8246789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8fb2982ff","size":5,"soft_fail":false,"target_va":5374220064,"va":5382466848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140541e30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8244185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8273482ff","size":5,"soft_fail":false,"target_va":5374221872,"va":5382466052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7730632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8380a8aff","size":5,"soft_fail":false,"target_va":5374736096,"va":5382466723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065d060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7084772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e81ce593ff","size":5,"soft_fail":false,"target_va":5375381600,"va":5382466367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6530056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8f85b9cff","size":5,"soft_fail":false,"target_va":5375936112,"va":5382466163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7a1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t374240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8e0b50500","size":5,"soft_fail":false,"target_va":5382840752,"va":5382466507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141008720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3054842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8fa9c2e00","size":5,"soft_fail":false,"target_va":5385520928,"va":5382466081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21127754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e84a624201","size":5,"soft_fail":false,"target_va":5403594336,"va":5382466577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4936920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e828abb4ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5382466803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8107738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8264984ff","size":5,"soft_fail":false,"target_va":5374359200,"va":5382466933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5100010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382465664,"raw_hex":"e8162eb2ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5382466229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":17284,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":16936,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1302,"record_offset":17156,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1302,"record_offset":17100,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":16964,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1302,"record_offset":17044,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":24591,"source_line":28,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::prefab::PrefabLibrary::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/prefab.rs
//     evidence={"module":1302,"record_offset":15952,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":23314,"source_line":45,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_atlases::generate_stone_floor_meshes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mesh_atlases.rs
//     evidence={"module":1302,"record_offset":17424,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":60340,"source_line":155,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":16328,"parent_record_offset":15776,"ancestor_record_offset":15776,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":15776,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":18744,"parent_record_offset":18688,"ancestor_record_offset":15776,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":18532,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":18580,"parent_record_offset":18556,"ancestor_record_offset":15776,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":18716,"parent_record_offset":18688,"ancestor_record_offset":15776,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":15776,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":17184,"parent_record_offset":17156,"ancestor_record_offset":15776,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":15776,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":18216,"parent_record_offset":18192,"ancestor_record_offset":15776,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":18276,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1302,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":15776,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18016,"parent_record_offset":17988,"ancestor_record_offset":15776,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":15776,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18044,"parent_record_offset":17988,"ancestor_record_offset":15776,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1302,"record_offset":17608,"parent_record_offset":17548,"ancestor_record_offset":15776,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":16208,"parent_record_offset":16180,"ancestor_record_offset":15776,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":16268,"parent_record_offset":16236,"ancestor_record_offset":15776,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":18472,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":15776,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":17524,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1302,"record_offset":18240,"parent_record_offset":18216,"ancestor_record_offset":15776,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":18192,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1302,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":15776,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1302,"record_offset":16552,"parent_record_offset":16528,"ancestor_record_offset":15776,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1302,"record_offset":17580,"parent_record_offset":17548,"ancestor_record_offset":15776,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":15776,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":18556,"parent_record_offset":18532,"ancestor_record_offset":15776,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":16152,"parent_record_offset":16124,"ancestor_record_offset":15776,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":18616,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":16772,"parent_record_offset":16748,"ancestor_record_offset":15776,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":16748,"parent_record_offset":16724,"ancestor_record_offset":15776,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1302,"record_offset":17008,"parent_record_offset":16964,"ancestor_record_offset":15776,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":15776,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":18304,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":17220,"parent_record_offset":17184,"ancestor_record_offset":15776,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":17244,"parent_record_offset":17220,"ancestor_record_offset":15776,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1302,"record_offset":17692,"parent_record_offset":17648,"ancestor_record_offset":15776,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1302,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":15776,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_terrain::generate_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1302,"record_offset":17648,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":24286,"source_line":253,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":15776,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":15776,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16724,"parent_record_offset":16416,"ancestor_record_offset":15776,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16124,"parent_record_offset":16064,"ancestor_record_offset":15776,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16416,"parent_record_offset":16384,"ancestor_record_offset":15776,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16036,"parent_record_offset":16008,"ancestor_record_offset":15776,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16096,"parent_record_offset":16064,"ancestor_record_offset":15776,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16064,"parent_record_offset":16036,"ancestor_record_offset":15776,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":16496,"parent_record_offset":16464,"ancestor_record_offset":15776,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":16528,"parent_record_offset":16496,"ancestor_record_offset":15776,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":16464,"parent_record_offset":16416,"ancestor_record_offset":15776,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":15776,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":16180,"parent_record_offset":16124,"ancestor_record_offset":15776,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":16236,"parent_record_offset":16180,"ancestor_record_offset":15776,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":15980,"parent_record_offset":15952,"ancestor_record_offset":15776,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":15776,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18444,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18416,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":21926,"source_line":222,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18388,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18504,"parent_record_offset":17424,"ancestor_record_offset":15776,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":15776,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1302,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":15776,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17988,"parent_record_offset":17840,"ancestor_record_offset":15776,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":15776,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1302,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":15776,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1302,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":15776,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1302,"record_offset":17840,"parent_record_offset":17816,"ancestor_record_offset":15776,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1302,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":15776,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1386,"record_offset":49112,"rva":16811808,"va":5385520928}
// 编译实例 RVAs: 0x1008720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15908233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e877420dff","size":5,"soft_fail":false,"target_va":5369614224,"va":5385522452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15907499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e855450dff","size":5,"soft_fail":false,"target_va":5369614592,"va":5385522086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036bdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13228364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8b42636ff","size":5,"soft_fail":false,"target_va":5372296688,"va":5385525047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036efd0
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13215728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8105836ff","size":5,"soft_fail":false,"target_va":5372309456,"va":5385525179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13210177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8bf6d36ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5385524380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12771425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e89f1f3dff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385521116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12772461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8931b3dff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385522168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12773058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e83e193dff","size":5,"soft_fail":false,"target_va":5372749744,"va":5385522797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140541720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11303194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8e68653ff","size":5,"soft_fail":false,"target_va":5374220064,"va":5385523253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b7b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10817332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8ccf05aff","size":5,"soft_fail":false,"target_va":5374704464,"va":5385521791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10786883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8bd675bff","size":5,"soft_fail":false,"target_va":5374736096,"va":5385522974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bfc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10785547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8f56c5bff","size":5,"soft_fail":false,"target_va":5374737536,"va":5385523078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c0220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10783886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e872735bff","size":5,"soft_fail":false,"target_va":5374738976,"va":5385522857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c07c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10782722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8fe775bff","size":5,"soft_fail":false,"target_va":5374740416,"va":5385523133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c0d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10781337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8677d5bff","size":5,"soft_fail":false,"target_va":5374741856,"va":5385523188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dab20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9627248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e890196dff","size":5,"soft_fail":false,"target_va":5375896352,"va":5385523595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9594209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e89f9a6dff","size":5,"soft_fail":false,"target_va":5375929952,"va":5385524156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9588673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e83fb06dff","size":5,"soft_fail":false,"target_va":5375936112,"va":5385524780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ea750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9560441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8871e6eff","size":5,"soft_fail":false,"target_va":5375960912,"va":5385521348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2712270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8329dd6ff","size":5,"soft_fail":false,"target_va":5382810688,"va":5385522953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2711737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8479fd6ff","size":5,"soft_fail":false,"target_va":5382811104,"va":5385522836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2193989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8bb85deff","size":5,"soft_fail":false,"target_va":5383329296,"va":5385523280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df3700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2184384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e840abdeff","size":5,"soft_fail":false,"target_va":5383337728,"va":5385522107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141082130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t497897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8e9980700","size":5,"soft_fail":false,"target_va":5386019120,"va":5385521218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mesh_atlases.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18070694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8a6bc1301","size":5,"soft_fail":false,"target_va":5403593344,"va":5385522645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t17454467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e883550a01","size":5,"soft_fail":false,"target_va":5402978144,"va":5385523672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7991868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8c40d86ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5385521751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7996535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e889fb85ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5385523650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8160337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8af7b83ff","size":5,"soft_fail":false,"target_va":5377364336,"va":5385524668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17455066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8da570a01","size":5,"soft_fail":false,"target_va":5402978944,"va":5385523873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17456325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8c55c0a01","size":5,"soft_fail":false,"target_va":5402980128,"va":5385523798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8113071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385520928,"raw_hex":"e8513484ff","size":5,"soft_fail":false,"target_va":5377408016,"va":5385521082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*25760286(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385520928,"raw_hex":"ff151e128901","size":6,"soft_fail":false,"va":5385524268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1386,"record_offset":50524,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1386,"record_offset":54156,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1386,"record_offset":50160,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1386,"record_offset":51724,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1386,"record_offset":50216,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1386,"record_offset":50312,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1386,"record_offset":50608,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1386,"record_offset":57348,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1386,"record_offset":52236,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1386,"record_offset":49516,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1386,"record_offset":50692,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1386,"record_offset":51836,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1386,"record_offset":53032,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":26960,"source_line":36,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1386,"record_offset":50764,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":23397,"source_line":205,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_reader
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1386,"record_offset":50056,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":63495,"source_line":98,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":51900,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1386,"record_offset":49400,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::BufReader::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1386,"record_offset":49560,"parent_record_offset":49112,"ancestor_record_offset":49112,"inlinee":20660,"source_line":72,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":49772,"parent_record_offset":49748,"ancestor_record_offset":49112,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":49796,"parent_record_offset":49772,"ancestor_record_offset":49112,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":54336,"parent_record_offset":54312,"ancestor_record_offset":49112,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":52180,"parent_record_offset":52156,"ancestor_record_offset":49112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":49748,"parent_record_offset":49720,"ancestor_record_offset":49112,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":54288,"parent_record_offset":54252,"ancestor_record_offset":49112,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1386,"record_offset":52156,"parent_record_offset":52100,"ancestor_record_offset":49112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$2::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1386,"record_offset":49664,"parent_record_offset":49636,"ancestor_record_offset":49112,"inlinee":5190,"source_line":636,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1386,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":49112,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":52128,"parent_record_offset":52100,"ancestor_record_offset":49112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":49720,"parent_record_offset":49692,"ancestor_record_offset":49112,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":49692,"parent_record_offset":49664,"ancestor_record_offset":49112,"inlinee":5191,"source_line":116,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":54220,"parent_record_offset":54188,"ancestor_record_offset":49112,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":49868,"parent_record_offset":49692,"ancestor_record_offset":49112,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1386,"record_offset":52100,"parent_record_offset":52072,"ancestor_record_offset":49112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1386,"record_offset":56792,"parent_record_offset":56768,"ancestor_record_offset":49112,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1386,"record_offset":57128,"parent_record_offset":57104,"ancestor_record_offset":49112,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1386,"record_offset":54188,"parent_record_offset":54156,"ancestor_record_offset":49112,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1386,"record_offset":53952,"parent_record_offset":53924,"ancestor_record_offset":49112,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1386,"record_offset":51096,"parent_record_offset":51068,"ancestor_record_offset":49112,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1386,"record_offset":53312,"parent_record_offset":53288,"ancestor_record_offset":49112,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1386,"record_offset":51148,"parent_record_offset":51124,"ancestor_record_offset":49112,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1386,"record_offset":53336,"parent_record_offset":53312,"ancestor_record_offset":49112,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1386,"record_offset":52384,"parent_record_offset":52356,"ancestor_record_offset":49112,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1386,"record_offset":51028,"parent_record_offset":51004,"ancestor_record_offset":49112,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1386,"record_offset":51688,"parent_record_offset":50764,"ancestor_record_offset":49112,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1386,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":49112,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1386,"record_offset":49908,"parent_record_offset":49596,"ancestor_record_offset":49112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1386,"record_offset":50184,"parent_record_offset":50160,"ancestor_record_offset":49112,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1386,"record_offset":51232,"parent_record_offset":51208,"ancestor_record_offset":49112,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1386,"record_offset":51208,"parent_record_offset":51184,"ancestor_record_offset":49112,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1386,"record_offset":50280,"parent_record_offset":50216,"ancestor_record_offset":49112,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1386,"record_offset":50248,"parent_record_offset":50216,"ancestor_record_offset":49112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1386,"record_offset":49824,"parent_record_offset":49796,"ancestor_record_offset":49112,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1386,"record_offset":54036,"parent_record_offset":54008,"ancestor_record_offset":49112,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1386,"record_offset":54064,"parent_record_offset":54036,"ancestor_record_offset":49112,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1386,"record_offset":51748,"parent_record_offset":51724,"ancestor_record_offset":49112,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1386,"record_offset":51772,"parent_record_offset":51748,"ancestor_record_offset":49112,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1386,"record_offset":52972,"parent_record_offset":52908,"ancestor_record_offset":49112,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1386,"record_offset":53980,"parent_record_offset":53952,"ancestor_record_offset":49112,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1386,"record_offset":50340,"parent_record_offset":50312,"ancestor_record_offset":49112,"inlinee":22866,"source_line":30,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1386,"record_offset":53924,"parent_record_offset":53896,"ancestor_record_offset":49112,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":53080,"parent_record_offset":53056,"ancestor_record_offset":49112,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":53104,"parent_record_offset":53080,"ancestor_record_offset":49112,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":51420,"parent_record_offset":51396,"ancestor_record_offset":49112,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":51924,"parent_record_offset":51900,"ancestor_record_offset":49112,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":51956,"parent_record_offset":51924,"ancestor_record_offset":49112,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":53896,"parent_record_offset":53676,"ancestor_record_offset":49112,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":50844,"parent_record_offset":50816,"ancestor_record_offset":49112,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1386,"record_offset":51984,"parent_record_offset":51956,"ancestor_record_offset":49112,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53808,"parent_record_offset":53784,"ancestor_record_offset":49112,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53784,"parent_record_offset":53676,"ancestor_record_offset":49112,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53724,"parent_record_offset":53700,"ancestor_record_offset":49112,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":51184,"parent_record_offset":50904,"ancestor_record_offset":49112,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53644,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":51524,"parent_record_offset":51468,"ancestor_record_offset":49112,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":50904,"parent_record_offset":50872,"ancestor_record_offset":49112,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53700,"parent_record_offset":53676,"ancestor_record_offset":49112,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53184,"parent_record_offset":53160,"ancestor_record_offset":49112,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53160,"parent_record_offset":53104,"ancestor_record_offset":49112,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":51444,"parent_record_offset":51420,"ancestor_record_offset":49112,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":51496,"parent_record_offset":51468,"ancestor_record_offset":49112,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53208,"parent_record_offset":53184,"ancestor_record_offset":49112,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":51468,"parent_record_offset":51444,"ancestor_record_offset":49112,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1386,"record_offset":50972,"parent_record_offset":50944,"ancestor_record_offset":49112,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1386,"record_offset":51004,"parent_record_offset":50972,"ancestor_record_offset":49112,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1386,"record_offset":50944,"parent_record_offset":50904,"ancestor_record_offset":49112,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53260,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":55064,"parent_record_offset":55040,"ancestor_record_offset":49112,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":50872,"parent_record_offset":50844,"ancestor_record_offset":49112,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1386,"record_offset":53676,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1386,"record_offset":53288,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1386,"record_offset":51068,"parent_record_offset":50904,"ancestor_record_offset":49112,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1386,"record_offset":53372,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1386,"record_offset":53552,"parent_record_offset":53208,"ancestor_record_offset":49112,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1386,"record_offset":51124,"parent_record_offset":51068,"ancestor_record_offset":49112,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":1386,"record_offset":50080,"parent_record_offset":50056,"ancestor_record_offset":49112,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":53056,"parent_record_offset":53032,"ancestor_record_offset":49112,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":51396,"parent_record_offset":51372,"ancestor_record_offset":49112,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":51372,"parent_record_offset":50764,"ancestor_record_offset":49112,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":50816,"parent_record_offset":50788,"ancestor_record_offset":49112,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1386,"record_offset":50788,"parent_record_offset":50764,"ancestor_record_offset":49112,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1386,"record_offset":49424,"parent_record_offset":49400,"ancestor_record_offset":49112,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1386,"record_offset":49484,"parent_record_offset":49400,"ancestor_record_offset":49112,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1386,"record_offset":49596,"parent_record_offset":49560,"ancestor_record_offset":49112,"inlinee":5186,"source_line":93,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1386,"record_offset":49636,"parent_record_offset":49596,"ancestor_record_offset":49112,"inlinee":5188,"source_line":33,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1386,"record_offset":50004,"parent_record_offset":49980,"ancestor_record_offset":49112,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1386,"record_offset":50656,"parent_record_offset":50632,"ancestor_record_offset":49112,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1386,"record_offset":50632,"parent_record_offset":50608,"ancestor_record_offset":49112,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1386,"record_offset":49452,"parent_record_offset":49424,"ancestor_record_offset":49112,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/01386.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_stone_floor_meshes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEBB4
// 首条位置: {"module":1302,"inlinee":60340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":18532,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":18276,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":18472,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":17524,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":18192,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":18616,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":18304,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":1302,"record_offset":17648,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":24286,"source_line":253,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18444,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18416,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":21926,"source_line":222,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18388,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":18504,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":17424,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":18744,"parent_record_offset":18688,"ancestor_record_offset":17424,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":18580,"parent_record_offset":18556,"ancestor_record_offset":17424,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":18716,"parent_record_offset":18688,"ancestor_record_offset":17424,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":17424,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":18216,"parent_record_offset":18192,"ancestor_record_offset":17424,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18016,"parent_record_offset":17988,"ancestor_record_offset":17424,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":17424,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1302,"record_offset":18044,"parent_record_offset":17988,"ancestor_record_offset":17424,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1302,"record_offset":17608,"parent_record_offset":17548,"ancestor_record_offset":17424,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":17424,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1302,"record_offset":18240,"parent_record_offset":18216,"ancestor_record_offset":17424,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1302,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":17424,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1302,"record_offset":17580,"parent_record_offset":17548,"ancestor_record_offset":17424,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":17424,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":18556,"parent_record_offset":18532,"ancestor_record_offset":17424,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":18640,"parent_record_offset":18616,"ancestor_record_offset":17424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1302,"record_offset":18352,"parent_record_offset":18304,"ancestor_record_offset":17424,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17424,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1302,"record_offset":17692,"parent_record_offset":17648,"ancestor_record_offset":17424,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1302,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":17424,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17424,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":17424,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1302,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":17424,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17988,"parent_record_offset":17840,"ancestor_record_offset":17424,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1302,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17424,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1302,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":17424,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1302,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":17424,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1302,"record_offset":17840,"parent_record_offset":17816,"ancestor_record_offset":17424,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1302,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":17424,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_atlases::generate_stone_floor_meshes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF05D
// 首条位置: {"module":1308,"inlinee":61533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: load_mesh_atlases.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshAttribAdd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshAttribAdd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::MeshAttribAdd,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribAdd", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshAttribConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_mesh_atlases::MeshAttribConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::MeshConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::MeshConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::MeshConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::MeshConfig,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::MeshConfig", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_reader_seed<std::io::buffered::bufreader::BufReader<std::fs::File>,core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >,alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "constructor": "ron::options::Options::from_reader_seed", "arguments": ["std::io::buffered::bufreader::BufReader<std::fs::File>", "core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >", "alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_mesh_atlases.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_mesh_atlases.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize
// 信息 ID: 022cf7416f742b34206e246b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshConfig> >; module=1373, inline_record=14980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig> >; module=1373, inline_record=16636; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::MeshAttribAdd> >; module=1373, inline_record=19468; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=39980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=48072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: e11f09b6170e61a5e83583ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=39276; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=43248; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=46692; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=58316; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1373, inline_record=73272; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1386, inline_record=36740; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1386, inline_record=37160; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__FieldVisitor>; module=1386, inline_record=42068; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: b1084ab1ec5267c438ad19e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Field> > @ 0xf443f7 -> 0x1009a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 84a50230efe4893a42c236a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Field> >; module=1373, inline_record=3904; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=38772; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=38980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=39136; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=42744; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=42952; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=43108; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=46208; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 0c976c31e7f703ec36c1e1c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=30320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 0819159b010dcf228bd89a32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=38380; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=42352; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=45808; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=57404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_stone_floor_meshes
// 信息 ID: f074ac9a8d6415dc1f3c0d09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1302, inline_record=17424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::generate_stone_floor_meshes::closure$0
// 信息 ID: 1749f48e2c4dae6a306b2cdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mesh_atlases::generate_stone_floor_meshes", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_mesh_atlases::generate_stone_floor_meshes::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_stone_floor_meshes::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_stone_floor_meshes::closure_env$0> >; module=1308, inline_record=191944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::impl$0::default
// 信息 ID: e71f8aaf0962878781e1ee76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::startup::load_mesh_atlases::_::impl$0::deserialize::__Visitor>; module=1373, inline_record=49436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::load_mesh_atlases
// 信息 ID: f00f6185a63d16bf8435054d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 80ca3fe3bce2a0332ab9b699: void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0xe826ec -> 0xd1e880 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> @ 0x10052aa -> 0xd1e880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
// 信息 ID: 8af8932ecccd7040afc8ea4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_mesh_atlases::load_nani_meshes -> ["country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xd1ea21 -> 0x1008720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
