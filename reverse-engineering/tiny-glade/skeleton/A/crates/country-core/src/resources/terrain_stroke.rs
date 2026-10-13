// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/terrain_stroke.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 80d6acaaf0e434fdb09c984a356111e841908bb10a573d691f781e9ae587ac62
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_stroke.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 419, 463, 497, 1440
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":463,"record_offset":4444,"rva":6550080,"va":5375259200}
// 编译实例 RVAs: 0x63f240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-3953190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375259200,"raw_hex":"e8daadc3ff","size":5,"soft_fail":false,"target_va":5371306160,"va":5375259345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":4624,"parent_record_offset":4444,"ancestor_record_offset":4444,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":4720,"parent_record_offset":4444,"ancestor_record_offset":4444,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":5136,"parent_record_offset":4444,"ancestor_record_offset":4444,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":5048,"parent_record_offset":4444,"ancestor_record_offset":4444,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":4444,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":4660,"parent_record_offset":4624,"ancestor_record_offset":4444,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":4784,"parent_record_offset":4744,"ancestor_record_offset":4444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":5360,"parent_record_offset":5328,"ancestor_record_offset":4444,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":5016,"parent_record_offset":4916,"ancestor_record_offset":4444,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":4812,"parent_record_offset":4744,"ancestor_record_offset":4444,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":463,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4444,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":463,"record_offset":4848,"parent_record_offset":4812,"ancestor_record_offset":4444,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D0F
// 首条位置: {"module":1440,"inlinee":27919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":54884,"rva":7132272,"va":5375841392}
// 编译实例 RVAs: 0x6cd470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":55076,"parent_record_offset":54884,"ancestor_record_offset":54884,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":55100,"parent_record_offset":55076,"ancestor_record_offset":54884,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":55124,"parent_record_offset":55100,"ancestor_record_offset":54884,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":55148,"parent_record_offset":55124,"ancestor_record_offset":54884,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10448
// 首条位置: {"module":1440,"inlinee":66632}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":245312,"parent_record_offset":245284,"ancestor_record_offset":245284,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":245580,"parent_record_offset":245556,"ancestor_record_offset":245284,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":245556,"parent_record_offset":245500,"ancestor_record_offset":245284,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":245528,"parent_record_offset":245500,"ancestor_record_offset":245284,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":245500,"parent_record_offset":245476,"ancestor_record_offset":245284,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":245428,"parent_record_offset":245396,"ancestor_record_offset":245284,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":245396,"parent_record_offset":245368,"ancestor_record_offset":245284,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":245340,"parent_record_offset":245312,"ancestor_record_offset":245284,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":245368,"parent_record_offset":245340,"ancestor_record_offset":245284,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x675E
// 首条位置: {"module":497,"inlinee":26462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10446
// 首条位置: {"module":1440,"inlinee":66630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":246380,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":247864,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":246684,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":247752,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":243844,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":246104,"parent_record_offset":243800,"ancestor_record_offset":243800,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":244496,"parent_record_offset":244472,"ancestor_record_offset":243800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":244472,"parent_record_offset":244416,"ancestor_record_offset":243800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":245116,"parent_record_offset":245088,"ancestor_record_offset":243800,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":245176,"parent_record_offset":245116,"ancestor_record_offset":243800,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":243992,"parent_record_offset":243968,"ancestor_record_offset":243800,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":244044,"parent_record_offset":244016,"ancestor_record_offset":243800,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":244068,"parent_record_offset":244044,"ancestor_record_offset":243800,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":244444,"parent_record_offset":244416,"ancestor_record_offset":243800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":244628,"parent_record_offset":244604,"ancestor_record_offset":243800,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":244416,"parent_record_offset":244388,"ancestor_record_offset":243800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":244604,"parent_record_offset":244576,"ancestor_record_offset":243800,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":244576,"parent_record_offset":244544,"ancestor_record_offset":243800,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":245208,"parent_record_offset":245116,"ancestor_record_offset":243800,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":246220,"parent_record_offset":246196,"ancestor_record_offset":243800,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":244776,"parent_record_offset":244748,"ancestor_record_offset":243800,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":244016,"parent_record_offset":243992,"ancestor_record_offset":243800,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":246196,"parent_record_offset":246160,"ancestor_record_offset":243800,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":244284,"parent_record_offset":243916,"ancestor_record_offset":243800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":244144,"parent_record_offset":244120,"ancestor_record_offset":243800,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":245144,"parent_record_offset":245116,"ancestor_record_offset":243800,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":246276,"parent_record_offset":246220,"ancestor_record_offset":243800,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":245852,"parent_record_offset":243916,"ancestor_record_offset":243800,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":244096,"parent_record_offset":244068,"ancestor_record_offset":243800,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":244120,"parent_record_offset":244096,"ancestor_record_offset":243800,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1440,"record_offset":246976,"parent_record_offset":246952,"ancestor_record_offset":243800,"inlinee":24927,"source_line":28,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_stroke.rs
//     evidence={"module":1440,"record_offset":245284,"parent_record_offset":245256,"ancestor_record_offset":243800,"inlinee":66632,"source_line":9,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":247824,"parent_record_offset":247800,"ancestor_record_offset":243800,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":248056,"parent_record_offset":248032,"ancestor_record_offset":243800,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":247800,"parent_record_offset":247776,"ancestor_record_offset":243800,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":248192,"parent_record_offset":248168,"ancestor_record_offset":243800,"inlinee":24928,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":243880,"parent_record_offset":243844,"ancestor_record_offset":243800,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":246132,"parent_record_offset":246104,"ancestor_record_offset":243800,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":245396,"parent_record_offset":245368,"ancestor_record_offset":243800,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":246336,"parent_record_offset":246312,"ancestor_record_offset":243800,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":245256,"parent_record_offset":243916,"ancestor_record_offset":243800,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":247676,"parent_record_offset":247652,"ancestor_record_offset":243800,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":247000,"parent_record_offset":246976,"ancestor_record_offset":243800,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":246640,"parent_record_offset":246616,"ancestor_record_offset":243800,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":243968,"parent_record_offset":243916,"ancestor_record_offset":243800,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":245340,"parent_record_offset":245312,"ancestor_record_offset":243800,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":245312,"parent_record_offset":245284,"ancestor_record_offset":243800,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":245368,"parent_record_offset":245340,"ancestor_record_offset":243800,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":243916,"parent_record_offset":243880,"ancestor_record_offset":243800,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":246160,"parent_record_offset":246132,"ancestor_record_offset":243800,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1041E
// 首条位置: {"module":1440,"inlinee":66590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":62616,"parent_record_offset":62020,"ancestor_record_offset":62020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":62052,"parent_record_offset":62020,"ancestor_record_offset":62020,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":62804,"parent_record_offset":62780,"ancestor_record_offset":62020,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":62780,"parent_record_offset":62672,"ancestor_record_offset":62020,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":62708,"parent_record_offset":62672,"ancestor_record_offset":62020,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":62672,"parent_record_offset":62644,"ancestor_record_offset":62020,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":62140,"parent_record_offset":62108,"ancestor_record_offset":62020,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":62744,"parent_record_offset":62708,"ancestor_record_offset":62020,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":62644,"parent_record_offset":62616,"ancestor_record_offset":62020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":62200,"parent_record_offset":62140,"ancestor_record_offset":62020,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":62168,"parent_record_offset":62140,"ancestor_record_offset":62020,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":62288,"parent_record_offset":62108,"ancestor_record_offset":62020,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1440,"record_offset":63064,"parent_record_offset":63040,"ancestor_record_offset":62020,"inlinee":24927,"source_line":28,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":62080,"parent_record_offset":62052,"ancestor_record_offset":62020,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":62232,"parent_record_offset":62108,"ancestor_record_offset":62020,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":62256,"parent_record_offset":62232,"ancestor_record_offset":62020,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":63088,"parent_record_offset":63064,"ancestor_record_offset":62020,"inlinee":9849,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":62544,"parent_record_offset":62520,"ancestor_record_offset":62020,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":62108,"parent_record_offset":62080,"ancestor_record_offset":62020,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":54028,"rva":7131776,"va":5375840896}
// 编译实例 RVAs: 0x6cd280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3091510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8cad3d0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375841217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1058844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8e4d7efff","size":5,"soft_fail":false,"target_va":5374782544,"va":5375841383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t50679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8f7c50000","size":5,"soft_fail":false,"target_va":5375891872,"va":5375841188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t184707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e883d10200","size":5,"soft_fail":false,"target_va":5376025984,"va":5375841272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410960d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10259886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8ae8d9c00","size":5,"soft_fail":false,"target_va":5386100944,"va":5375841053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10263617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8419c9c00","size":5,"soft_fail":false,"target_va":5386104640,"va":5375841018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109afc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_stroke.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10280164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375840896,"raw_hex":"e8e4dc9c00","size":5,"soft_fail":false,"target_va":5386121152,"va":5375840983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":54244,"parent_record_offset":54028,"ancestor_record_offset":54028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":497,"record_offset":54216,"parent_record_offset":54028,"ancestor_record_offset":54028,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":54628,"parent_record_offset":54604,"ancestor_record_offset":54028,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":54604,"parent_record_offset":54548,"ancestor_record_offset":54028,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":497,"record_offset":54320,"parent_record_offset":54296,"ancestor_record_offset":54028,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":497,"record_offset":54376,"parent_record_offset":54320,"ancestor_record_offset":54028,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":54576,"parent_record_offset":54548,"ancestor_record_offset":54028,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":54548,"parent_record_offset":54520,"ancestor_record_offset":54028,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":497,"record_offset":54408,"parent_record_offset":54320,"ancestor_record_offset":54028,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":54272,"parent_record_offset":54244,"ancestor_record_offset":54028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":54344,"parent_record_offset":54320,"ancestor_record_offset":54028,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::terrain_stroke::TerrainStroke::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E6F
// 首条位置: {"module":419,"inlinee":24175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":16180,"parent_record_offset":16148,"ancestor_record_offset":16148,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":16232,"parent_record_offset":16208,"ancestor_record_offset":16148,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":16148,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":419,"record_offset":16208,"parent_record_offset":16180,"ancestor_record_offset":16148,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":16280,"parent_record_offset":16256,"ancestor_record_offset":16148,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00419.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_stroke.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::terrain_stroke::TerrainStroke
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "tuple$", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "core::option::Option", "arguments": ["country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::terrain_stroke::TerrainStroke>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::terrain_stroke::TerrainStroke>", "constructor": "ref$", "arguments": ["country_core::resources::terrain_stroke::TerrainStroke"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::terrain_editing::TerrainStrokeId", "country_core::resources::terrain_stroke::TerrainStroke", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "ref$", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::terrain_editing::TerrainStrokeId>,ref$<country_core::resources::terrain_stroke::TerrainStroke>,ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> > >", "constructor": "serde::ser::Serializer::collect_map", "arguments": ["serde_json::value::ser::Serializer", "ref$<country_core::resources::terrain_editing::TerrainStrokeId>", "ref$<country_core::resources::terrain_stroke::TerrainStroke>", "ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde_json::value::de::visit_object", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_stroke.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/terrain_stroke.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_stroke.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::TerrainStroke::insert_or_morph_last_point
// 信息 ID: 33e995aba09e0ab98ad47108
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::TerrainStroke::new
// 信息 ID: 9f5bdd1906905ed80364628f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::terrain_editing::TerrainEditingData::new_stroke; module=419, inline_record=16148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize
// 信息 ID: 270c4e5fe99f3a4e5362b336
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<serde::de::impls::impl$204::deserialize::MapVisitor<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> >; module=1440, inline_record=181076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: c30149c5880a8a925f1b0e8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_object<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor> @ 0x10cded6 -> 0x6cd470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: c21410377732bc29d2a97e82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=245284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 1be2c0349c1ed740ef1d4f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=497, inline_record=432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 496696792ba2f73b20c6c715
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=243800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 61c7ea6af98a788e10c6e49f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::terrain_stroke::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=62020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::terrain_stroke::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 645600b1408654e39c7857e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::terrain_stroke::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::Serializer::collect_map<serde_json::value::ser::Serializer,ref$<country_core::resources::terrain_editing::TerrainStrokeId>,ref$<country_core::resources::terrain_stroke::TerrainStroke>,ref$<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState> > > @ 0x108f04a -> 0x6cd280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
