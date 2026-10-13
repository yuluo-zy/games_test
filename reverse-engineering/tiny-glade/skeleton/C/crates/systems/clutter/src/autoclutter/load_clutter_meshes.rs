// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5563d17404643deb82cdd045aa94e5db986a1e5c0474fd4c2655e65c5079f810
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_clutter_meshes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1136, 2735, 2739, 4068
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 0, 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A786
// 首条位置: {"module":2735,"record_offset":2260,"rva":35274768,"va":5403983888}
// 编译实例 RVAs: 0x21a4010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28305434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8e61750fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403984117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35261915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e825f2e5fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403984102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c60b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17687369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8b71cf2fe","size":5,"soft_fail":false,"target_va":5386297520,"va":5403984884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428baee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7433607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8876d7100","size":5,"soft_fail":false,"target_va":5411417824,"va":5403984212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12635989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e855cfc000","size":5,"soft_fail":false,"target_va":5416620769,"va":5403984775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19360858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8a693d8fe","size":5,"soft_fail":false,"target_va":5384624080,"va":5403984933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12636870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8c6d2c000","size":5,"soft_fail":false,"target_va":5416621632,"va":5403984757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22520842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8f65ba8fe","size":5,"soft_fail":false,"target_va":5381463584,"va":5403984421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_radius_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22519682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e87e60a8fe","size":5,"soft_fail":false,"target_va":5381464720,"va":5403984397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library_replacing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24334068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e80cb18cfe","size":5,"soft_fail":false,"target_va":5379650480,"va":5403984543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::legacy_mod_mesh::upgrade_legacy_textured_clutter_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6806181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e85b2598ff","size":5,"soft_fail":false,"target_va":5397178176,"va":5403984352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21099904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8800abefe","size":5,"soft_fail":false,"target_va":5382884432,"va":5403984331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20660438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e82abfc4fe","size":5,"soft_fail":false,"target_va":5383323664,"va":5403984097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20659279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403983888,"raw_hex":"e8b1c3c4fe","size":5,"soft_fail":false,"target_va":5383324736,"va":5403984010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2735,"record_offset":2444,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2735,"record_offset":2864,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":2712,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":5727,"source_line":1390,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":2776,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":2836,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2735,"record_offset":2636,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":2260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":2260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":2260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":2260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2735,"record_offset":2480,"parent_record_offset":2444,"ancestor_record_offset":2260,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2735,"record_offset":2892,"parent_record_offset":2864,"ancestor_record_offset":2260,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2735,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":2260,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":2260,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2735,"record_offset":2680,"parent_record_offset":2636,"ancestor_record_offset":2260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::reload_clutter_meshes_after_mod_swap
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1065B
// 首条位置: {"module":1136,"inlinee":67163}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":1136,"record_offset":12252,"parent_record_offset":12020,"ancestor_record_offset":12020,"inlinee":27039,"source_line":112,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":1136,"record_offset":12052,"parent_record_offset":12020,"ancestor_record_offset":12020,"inlinee":27034,"source_line":102,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1136,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":12020,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":1136,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":12020,"inlinee":27040,"source_line":144,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":1136,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":12020,"inlinee":67164,"source_line":139,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":1136,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":12020,"inlinee":27037,"source_line":120,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1136,"record_offset":12156,"parent_record_offset":12100,"ancestor_record_offset":12020,"inlinee":27038,"source_line":265,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1136,"record_offset":12212,"parent_record_offset":12100,"ancestor_record_offset":12020,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1136,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":12020,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2735,"record_offset":268,"rva":35268000,"va":5403977120}
// 编译实例 RVAs: 0x21a25a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28305070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8521950fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403983769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34208675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e85d04f6fd","size":5,"soft_fail":false,"target_va":5369774336,"va":5403983006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d680
//     owner=unresolved_namespace; count=1
//     target_aliases=24 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25519747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e87d997afe","size":5,"soft_fail":false,"target_va":5378463360,"va":5403983102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410caa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17666816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8006df2fe","size":5,"soft_fail":false,"target_va":5386316320,"va":5403983131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421cae60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t164991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e87f840200","size":5,"soft_fail":false,"target_va":5404143200,"va":5403978204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_clutter_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13073437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e81d7cc700","size":5,"soft_fail":false,"target_va":5417057056,"va":5403983614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7430240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e860607100","size":5,"soft_fail":false,"target_va":5411413888,"va":5403983643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12637836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e88cd6c000","size":5,"soft_fail":false,"target_va":5416621536,"va":5403983695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::clutter_mesh_ty_data
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24907674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e866f083fe","size":5,"soft_fail":false,"target_va":5379075488,"va":5403983157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::decoclutter_category_items
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24906802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8cef383fe","size":5,"soft_fail":false,"target_va":5379076576,"va":5403983373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::register_builtin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24571412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8ec1189fe","size":5,"soft_fail":false,"target_va":5379412048,"va":5403983455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::register_builtin_hidden
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24568897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8bf1b89fe","size":5,"soft_fail":false,"target_va":5379414320,"va":5403983212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::set_placeholder
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24571609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8271189fe","size":5,"soft_fail":false,"target_va":5379411984,"va":5403983588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
//     owner=package_namespace_candidate; count=56
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403977120,"raw_hex":"e8de190000","size":5,"soft_fail":false,"target_va":5403983888,"va":5403977261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2735,"record_offset":728,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":2735,"record_offset":988,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":2735,"record_offset":784,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":2735,"record_offset":536,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2735,"record_offset":452,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2735,"record_offset":1016,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":1616,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2735,"record_offset":1644,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2735,"record_offset":1808,"parent_record_offset":1716,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2735,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2735,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2735,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2735,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":268,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2735,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":2735,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2735,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2735,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":268,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2735,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":268,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":2735,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":268,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2735,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":268,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2735,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":268,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2735,"record_offset":688,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2735,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":268,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2735,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":268,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2735,"record_offset":1776,"parent_record_offset":1744,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":2735,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":268,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":2735,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2735,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":268,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2735,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":268,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2735,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2735,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2735,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2735,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2735,"record_offset":1408,"parent_record_offset":1324,"ancestor_record_offset":268,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2735,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2735,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2735,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":2735,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":268,"inlinee":67168,"source_line":154,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":2735,"record_offset":624,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs
//     evidence={"module":2735,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":108420,"source_line":650,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20FE8
// 首条位置: {"module":4068,"inlinee":135144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4068,"record_offset":6340,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4068,"record_offset":6152,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4068,"record_offset":6312,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4068,"record_offset":7048,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs
//     evidence={"module":4068,"record_offset":6508,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":108422,"source_line":61,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":6096,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":6096,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":6096,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":6096,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":6096,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6096,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":6096,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4068,"record_offset":7200,"parent_record_offset":7144,"ancestor_record_offset":6096,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4068,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":6096,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4068,"record_offset":6852,"parent_record_offset":6508,"ancestor_record_offset":6096,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4068,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":6096,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4068,"record_offset":7172,"parent_record_offset":7144,"ancestor_record_offset":6096,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4068,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":6096,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4068,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":6096,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4068,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":6096,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4068,"record_offset":6792,"parent_record_offset":6508,"ancestor_record_offset":6096,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4068,"record_offset":6760,"parent_record_offset":6728,"ancestor_record_offset":6096,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4068,"record_offset":6664,"parent_record_offset":6508,"ancestor_record_offset":6096,"inlinee":5727,"source_line":1390,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4068,"record_offset":6692,"parent_record_offset":6664,"ancestor_record_offset":6096,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4068,"record_offset":6728,"parent_record_offset":6508,"ancestor_record_offset":6096,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4068,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":6096,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4068,"record_offset":6632,"parent_record_offset":6592,"ancestor_record_offset":6096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4068,"record_offset":6592,"parent_record_offset":6508,"ancestor_record_offset":6096,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/04068.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 639
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1065E
// 首条位置: {"module":1136,"inlinee":67166}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1136,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":34808,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1136,"record_offset":34856,"parent_record_offset":34832,"ancestor_record_offset":34808,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1136,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34808,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1136,"record_offset":34880,"parent_record_offset":34856,"ancestor_record_offset":34808,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":1136,"record_offset":34924,"parent_record_offset":34904,"ancestor_record_offset":34808,"inlinee":67168,"source_line":154,"archive":"_pdb/C/modules/01136.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 650
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A784
// 首条位置: {"module":2735,"inlinee":108420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2735,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":1252,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2735,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":1252,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2735,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":1252,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2735,"record_offset":1408,"parent_record_offset":1324,"ancestor_record_offset":1252,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2735,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":1252,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2735,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1252,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02735.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":2735,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1252,"inlinee":67168,"source_line":154,"archive":"_pdb/C/modules/02735.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 29; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBeamAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBeamAlloc>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBeamAlloc>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes
// 信息 ID: 7154de5e6528ce22934d7a42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d2f8d13af56a97024378c3be: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f9bede89bd342bc352635ea5: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> @ 0x10c9dc8 -> 0x21a25a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> @ 0x10ca11a -> 0x21a25a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> > @ 0x21aff9f -> 0x21a25a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> > @ 0x21b1c01 -> 0x21a25a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0
// 信息 ID: 35de48005edde42234a63de9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// 引用方: 原程序直接转移 56 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a262d -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a267e -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a26cf -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2720 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2771 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a27c2 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2813 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2864 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a28b5 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2906 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2957 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a29a8 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2a2e -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2a87 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2ae0 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes @ 0x21a2b39 -> 0x21a4010 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; module=4068, inline_record=6508; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$1
// 信息 ID: f47f87c9226c1bb86e345d13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<country_core::resources::clutter_meshes::ClutterMesh>,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1 <- core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > >; module=4068, inline_record=6096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$2
// 信息 ID: 908ab51a35c8a73997bd94b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$2 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_meshes::ClutterMeshTy> >,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_meshes::ClutterMeshTy> >,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$2>; module=1136, inline_record=34808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$3
// 信息 ID: 2a5a179f6eef3490623eb35c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes; module=2735, inline_record=1252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::load_clutter_meshes::reload_clutter_meshes_after_mod_swap
// 信息 ID: 4a72b25f73b4aace86bdd72b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d2f8d13af56a97024378c3be: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>; module=1136, inline_record=12020; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >; module=2739, inline_record=353512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
