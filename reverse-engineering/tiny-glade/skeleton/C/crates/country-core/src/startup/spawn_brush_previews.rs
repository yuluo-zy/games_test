// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/spawn_brush_previews.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5b844bb69bba346002025d2d35680d05c091fc5c6cee9f13232028ab2db616af
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_brush_previews.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1636
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_brush_previews::spawn_brush_previews
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1636,"record_offset":84516,"rva":22094048,"va":5390803168}
// 编译实例 RVAs: 0x15120e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15138420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e88c0119ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390817119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12215846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8da9945ff","size":5,"soft_fail":false,"target_va":5378599808,"va":5390815649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12080946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8cea847ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5390817357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10788877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8f35f5bff","size":5,"soft_fail":false,"target_va":5380014704,"va":5390803576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c2eef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9329143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e809a671ff","size":5,"soft_fail":false,"target_va":5381484272,"va":5390813410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0980
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7488797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8e3ba8dff","size":5,"soft_fail":false,"target_va":5383326080,"va":5390814872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t934308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8a4410e00","size":5,"soft_fail":false,"target_va":5391737856,"va":5390803543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162cc70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1148313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e899851100","size":5,"soft_fail":false,"target_va":5391961200,"va":5390812882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162cee0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1152272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e810951100","size":5,"soft_fail":false,"target_va":5391961824,"va":5390809547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162dea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1157878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8f6aa1100","size":5,"soft_fail":false,"target_va":5391965856,"va":5390807973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162e110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1154463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e89f9d1100","size":5,"soft_fail":false,"target_va":5391966480,"va":5390812012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162e860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1153294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e80e991100","size":5,"soft_fail":false,"target_va":5391968352,"va":5390815053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ebc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1153572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8249a1100","size":5,"soft_fail":false,"target_va":5391969216,"va":5390815639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162fc00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1165070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e80ec71100","size":5,"soft_fail":false,"target_va":5391973376,"va":5390808301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1164861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e83dc61100","size":5,"soft_fail":false,"target_va":5391974000,"va":5390809134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1163649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e881c11100","size":5,"soft_fail":false,"target_va":5391975248,"va":5390811594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1162358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e876bc1100","size":5,"soft_fail":false,"target_va":5391976736,"va":5390814373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=11
//     target_aliases=3 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6140524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e86cb25d00","size":5,"soft_fail":false,"target_va":5396943824,"va":5390803295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aedab0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6138397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e81daa5d00","size":5,"soft_fail":false,"target_va":5396945584,"va":5390807182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: spawn_brush_previews.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25536749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8eda88501","size":5,"soft_fail":false,"target_va":5416353040,"va":5390816286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6012289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e87f42a4ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5390807148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6193202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e8ce7fa1ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5390817277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t25805883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e83bc48901","size":5,"soft_fail":false,"target_va":5416621632,"va":5390815744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11281514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e896db53ff","size":5,"soft_fail":false,"target_va":5379521760,"va":5390803269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11154082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390803168,"raw_hex":"e85ecd55ff","size":5,"soft_fail":false,"target_va":5379649664,"va":5390803741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1636,"record_offset":91896,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::remove
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1636,"record_offset":96620,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":11389,"source_line":1659,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1636,"record_offset":91576,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1636,"record_offset":91980,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1636,"record_offset":84700,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::render_pipelines_impl::impl$2::clone
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":1636,"record_offset":92980,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":32867,"source_line":61,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::with_color
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1636,"record_offset":84772,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":33733,"source_line":126,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation_scale
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1636,"record_offset":127592,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":32870,"source_line":66,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1636,"record_offset":94772,"parent_record_offset":84516,"ancestor_record_offset":84516,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1636,"record_offset":122520,"parent_record_offset":122496,"ancestor_record_offset":84516,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1636,"record_offset":122544,"parent_record_offset":122520,"ancestor_record_offset":84516,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1636,"record_offset":122568,"parent_record_offset":122544,"ancestor_record_offset":84516,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1636,"record_offset":122496,"parent_record_offset":122440,"ancestor_record_offset":84516,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1636,"record_offset":91944,"parent_record_offset":91920,"ancestor_record_offset":84516,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":96772,"parent_record_offset":96744,"ancestor_record_offset":84516,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":96856,"parent_record_offset":96832,"ancestor_record_offset":84516,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":122468,"parent_record_offset":122440,"ancestor_record_offset":84516,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":122440,"parent_record_offset":122416,"ancestor_record_offset":84516,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":96832,"parent_record_offset":96800,"ancestor_record_offset":84516,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":96800,"parent_record_offset":96772,"ancestor_record_offset":84516,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1636,"record_offset":122416,"parent_record_offset":122392,"ancestor_record_offset":84516,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1636,"record_offset":96744,"parent_record_offset":96708,"ancestor_record_offset":84516,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_handled
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1636,"record_offset":96676,"parent_record_offset":96648,"ancestor_record_offset":84516,"inlinee":11370,"source_line":639,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1636,"record_offset":96708,"parent_record_offset":96676,"ancestor_record_offset":84516,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue_handled
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1636,"record_offset":96648,"parent_record_offset":96620,"ancestor_record_offset":84516,"inlinee":11369,"source_line":1934,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1636,"record_offset":91604,"parent_record_offset":91576,"ancestor_record_offset":84516,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1636,"record_offset":91736,"parent_record_offset":91632,"ancestor_record_offset":84516,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1636,"record_offset":85128,"parent_record_offset":85104,"ancestor_record_offset":84516,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1636,"record_offset":91704,"parent_record_offset":91680,"ancestor_record_offset":84516,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1636,"record_offset":93208,"parent_record_offset":93180,"ancestor_record_offset":84516,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1636,"record_offset":85404,"parent_record_offset":85380,"ancestor_record_offset":84516,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1636,"record_offset":91680,"parent_record_offset":91632,"ancestor_record_offset":84516,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1636,"record_offset":91632,"parent_record_offset":91604,"ancestor_record_offset":84516,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1636,"record_offset":85632,"parent_record_offset":84896,"ancestor_record_offset":84516,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1636,"record_offset":91920,"parent_record_offset":91896,"ancestor_record_offset":84516,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=68
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1636,"record_offset":93372,"parent_record_offset":93008,"ancestor_record_offset":84516,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1636,"record_offset":91820,"parent_record_offset":91760,"ancestor_record_offset":84516,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1636,"record_offset":84740,"parent_record_offset":84700,"ancestor_record_offset":84516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1636,"record_offset":118564,"parent_record_offset":118532,"ancestor_record_offset":84516,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1636,"record_offset":85524,"parent_record_offset":85500,"ancestor_record_offset":84516,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1636,"record_offset":93180,"parent_record_offset":93156,"ancestor_record_offset":84516,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1636,"record_offset":93236,"parent_record_offset":93180,"ancestor_record_offset":84516,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1636,"record_offset":93260,"parent_record_offset":93236,"ancestor_record_offset":84516,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1636,"record_offset":93132,"parent_record_offset":93108,"ancestor_record_offset":84516,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1636,"record_offset":93156,"parent_record_offset":93132,"ancestor_record_offset":84516,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1636,"record_offset":91844,"parent_record_offset":91820,"ancestor_record_offset":84516,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1636,"record_offset":91760,"parent_record_offset":91736,"ancestor_record_offset":84516,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1636,"record_offset":91792,"parent_record_offset":91760,"ancestor_record_offset":84516,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1636,"record_offset":93308,"parent_record_offset":93080,"ancestor_record_offset":84516,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=85
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1636,"record_offset":93332,"parent_record_offset":93308,"ancestor_record_offset":84516,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$7::clone
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1636,"record_offset":93008,"parent_record_offset":92980,"ancestor_record_offset":84516,"inlinee":32855,"source_line":17,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::add_color
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1636,"record_offset":84808,"parent_record_offset":84772,"ancestor_record_offset":84516,"inlinee":30460,"source_line":115,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$0::vertex_count
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1636,"record_offset":84896,"parent_record_offset":84808,"ancestor_record_offset":84516,"inlinee":30462,"source_line":22,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1636,"record_offset":85348,"parent_record_offset":85320,"ancestor_record_offset":84516,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1636,"record_offset":85380,"parent_record_offset":85348,"ancestor_record_offset":84516,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1636,"record_offset":85320,"parent_record_offset":85280,"ancestor_record_offset":84516,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1636,"record_offset":85104,"parent_record_offset":85080,"ancestor_record_offset":84516,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1636,"record_offset":84864,"parent_record_offset":84840,"ancestor_record_offset":84516,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1636,"record_offset":84976,"parent_record_offset":84952,"ancestor_record_offset":84516,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1636,"record_offset":85232,"parent_record_offset":85208,"ancestor_record_offset":84516,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85500,"parent_record_offset":85280,"ancestor_record_offset":84516,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85080,"parent_record_offset":85024,"ancestor_record_offset":84516,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85280,"parent_record_offset":85256,"ancestor_record_offset":84516,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85000,"parent_record_offset":84976,"ancestor_record_offset":84516,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85052,"parent_record_offset":85024,"ancestor_record_offset":84516,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85024,"parent_record_offset":85000,"ancestor_record_offset":84516,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1636,"record_offset":85256,"parent_record_offset":85232,"ancestor_record_offset":84516,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$26::clone
//     owner=git_dependency_definition; count=17
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1636,"record_offset":94444,"parent_record_offset":94416,"ancestor_record_offset":84516,"inlinee":16899,"source_line":78,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=84
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1636,"record_offset":93108,"parent_record_offset":93080,"ancestor_record_offset":84516,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::chunk_ref_counts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1636,"record_offset":118532,"parent_record_offset":118304,"ancestor_record_offset":84516,"inlinee":5246,"source_line":444,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=85
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1636,"record_offset":93080,"parent_record_offset":93056,"ancestor_record_offset":84516,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$44::clone
//     owner=git_dependency_definition; count=68
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":1636,"record_offset":93056,"parent_record_offset":93008,"ancestor_record_offset":84516,"inlinee":32857,"source_line":78,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1636,"record_offset":84840,"parent_record_offset":84808,"ancestor_record_offset":84516,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1636,"record_offset":84952,"parent_record_offset":84928,"ancestor_record_offset":84516,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1636,"record_offset":84928,"parent_record_offset":84896,"ancestor_record_offset":84516,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1636,"record_offset":85208,"parent_record_offset":85184,"ancestor_record_offset":84516,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1636,"record_offset":85184,"parent_record_offset":84896,"ancestor_record_offset":84516,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/01636.jsonl.gz"}
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
// 完整结构及原始类型记录: spawn_brush_previews.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spawn_brush_previews.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spawn_brush_previews.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_brush_previews::spawn_brush_previews
// 信息 ID: 899fade88185740757c8d33f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8add82f21108b4958f4092da: void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x194d4ab -> 0x15120e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)> @ 0x1ab590c -> 0x15120e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
