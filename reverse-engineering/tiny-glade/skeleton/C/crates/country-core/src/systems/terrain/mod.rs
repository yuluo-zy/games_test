// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/terrain/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 546ed24e999b48b680a0beeea4b62bcef9eea7bc18faf9258f463a4d85135afe
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 511
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::terrain::impl$0::add_terrain_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":219080,"rva":9744624,"va":5378453744}
// 编译实例 RVAs: 0x94b0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5765018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e89af75700","size":5,"soft_fail":false,"target_va":5384218864,"va":5378453841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2775109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8bba7d5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378453792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9731590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8fa816bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378453777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=2
//     target_aliases=118 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9248589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8b3e072ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5378454248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7856012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8742088ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5378453911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8d4b30000","size":5,"soft_fail":false,"target_va":5378499936,"va":5378453895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141519790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12379373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8ede4bc00","size":5,"soft_fail":false,"target_va":5390833552,"va":5378454174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417dfbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15288927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e85f4ae900","size":5,"soft_fail":false,"target_va":5393742800,"va":5378453868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e39e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15304827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e87b88e900","size":5,"soft_fail":false,"target_va":5393758688,"va":5378453856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419ebeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17435828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e8b40c0a01","size":5,"soft_fail":false,"target_va":5395889840,"va":5378454007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38166560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378453744,"raw_hex":"e820604602","size":5,"soft_fail":false,"target_va":5416620769,"va":5378454204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":219564,"parent_record_offset":219080,"ancestor_record_offset":219080,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":219232,"parent_record_offset":219080,"ancestor_record_offset":219080,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":219492,"parent_record_offset":219080,"ancestor_record_offset":219080,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":219388,"parent_record_offset":219364,"ancestor_record_offset":219080,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":219412,"parent_record_offset":219388,"ancestor_record_offset":219080,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":219436,"parent_record_offset":219412,"ancestor_record_offset":219080,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":219364,"parent_record_offset":219332,"ancestor_record_offset":219080,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":219332,"parent_record_offset":219296,"ancestor_record_offset":219080,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":219296,"parent_record_offset":219260,"ancestor_record_offset":219080,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":219260,"parent_record_offset":219232,"ancestor_record_offset":219080,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":511,"record_offset":219532,"parent_record_offset":219492,"ancestor_record_offset":219080,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::terrain::impl$0::insert_terrain_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":219596,"rva":9745200,"va":5378454320}
// 编译实例 RVAs: 0x94b330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2775752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e838a5d5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378454435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2776115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8cda3d5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378454814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9732233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8777f6bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378454420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f1a10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13264443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e83b66ca00","size":5,"soft_fail":false,"target_va":5391718928,"va":5378454480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141853830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15762309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e88583f000","size":5,"soft_fail":false,"target_va":5394217008,"va":5378454694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141858bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15783675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8fbd6f000","size":5,"soft_fail":false,"target_va":5394238384,"va":5378454704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141862050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15821870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e82e6cf100","size":5,"soft_fail":false,"target_va":5394276432,"va":5378454557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141866a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15840959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8bfb6f100","size":5,"soft_fail":false,"target_va":5394295376,"va":5378454412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14186a270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15855353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8f9eef100","size":5,"soft_fail":false,"target_va":5394309744,"va":5378454386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14186cfd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15866627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8031bf200","size":5,"soft_fail":false,"target_va":5394321360,"va":5378454728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14186e5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15872649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e88932f200","size":5,"soft_fail":false,"target_va":5394327024,"va":5378454370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38166032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378454320,"raw_hex":"e8105e4602","size":5,"soft_fail":false,"target_va":5416620794,"va":5378454757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::terrain_height::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":511,"record_offset":219840,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29538,"source_line":100,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":511,"record_offset":220504,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29540,"source_line":111,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::animate_terrain_delete::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/animate_terrain_delete.rs
//     evidence={"module":511,"record_offset":219792,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29534,"source_line":24,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::delayed_terrain_changed_msg::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs
//     evidence={"module":511,"record_offset":221168,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29542,"source_line":26,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::ui_edit_terrain::MouseActivityTimer::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/ui_edit_terrain.rs
//     evidence={"module":511,"record_offset":219764,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29532,"source_line":214,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::ui_edit_terrain::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/ui_edit_terrain.rs
//     evidence={"module":511,"record_offset":219816,"parent_record_offset":219596,"ancestor_record_offset":219596,"inlinee":29536,"source_line":22,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220068,"parent_record_offset":220044,"ancestor_record_offset":219596,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220092,"parent_record_offset":220068,"ancestor_record_offset":219596,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220360,"parent_record_offset":220336,"ancestor_record_offset":219596,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220384,"parent_record_offset":220360,"ancestor_record_offset":219596,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220116,"parent_record_offset":220092,"ancestor_record_offset":219596,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220408,"parent_record_offset":220384,"ancestor_record_offset":219596,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220044,"parent_record_offset":220016,"ancestor_record_offset":219596,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":220336,"parent_record_offset":220244,"ancestor_record_offset":219596,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":219968,"parent_record_offset":219944,"ancestor_record_offset":219596,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":220272,"parent_record_offset":220244,"ancestor_record_offset":219596,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":220244,"parent_record_offset":220220,"ancestor_record_offset":219596,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":220016,"parent_record_offset":219992,"ancestor_record_offset":219596,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":219992,"parent_record_offset":219968,"ancestor_record_offset":219596,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":220220,"parent_record_offset":220196,"ancestor_record_offset":219596,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":219944,"parent_record_offset":219904,"ancestor_record_offset":219596,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":219876,"parent_record_offset":219840,"ancestor_record_offset":219596,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":511,"record_offset":219904,"parent_record_offset":219876,"ancestor_record_offset":219596,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":511,"record_offset":220304,"parent_record_offset":220272,"ancestor_record_offset":219596,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":220172,"parent_record_offset":219904,"ancestor_record_offset":219596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":511,"record_offset":220472,"parent_record_offset":219840,"ancestor_record_offset":219596,"inlinee":20485,"source_line":50,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::terrain::impl$0::add_terrain_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":208928,"rva":9743152,"va":5378452272}
// 编译实例 RVAs: 0x94ab30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15667510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378452272,"raw_hex":"e83611ef00","size":5,"soft_fail":false,"target_va":5394120032,"va":5378452517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":209072,"parent_record_offset":208928,"ancestor_record_offset":208928,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":210460,"parent_record_offset":210436,"ancestor_record_offset":208928,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":210508,"parent_record_offset":210484,"ancestor_record_offset":208928,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":210484,"parent_record_offset":210460,"ancestor_record_offset":208928,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":210436,"parent_record_offset":210412,"ancestor_record_offset":208928,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":210412,"parent_record_offset":210372,"ancestor_record_offset":208928,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":210372,"parent_record_offset":210348,"ancestor_record_offset":208928,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":511,"record_offset":209096,"parent_record_offset":209072,"ancestor_record_offset":208928,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":511,"record_offset":209156,"parent_record_offset":209124,"ancestor_record_offset":208928,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":210680,"parent_record_offset":210656,"ancestor_record_offset":208928,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":210656,"parent_record_offset":210632,"ancestor_record_offset":208928,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":210120,"parent_record_offset":210096,"ancestor_record_offset":208928,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":210168,"parent_record_offset":210144,"ancestor_record_offset":208928,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":210144,"parent_record_offset":210120,"ancestor_record_offset":208928,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":210348,"parent_record_offset":210324,"ancestor_record_offset":208928,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":209124,"parent_record_offset":209096,"ancestor_record_offset":208928,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":210096,"parent_record_offset":210072,"ancestor_record_offset":208928,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":511,"record_offset":209180,"parent_record_offset":209156,"ancestor_record_offset":208928,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":511,"record_offset":209820,"parent_record_offset":209796,"ancestor_record_offset":208928,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":511,"record_offset":209848,"parent_record_offset":209820,"ancestor_record_offset":208928,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":209396,"parent_record_offset":209372,"ancestor_record_offset":208928,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":209480,"parent_record_offset":209456,"ancestor_record_offset":208928,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":209628,"parent_record_offset":209600,"ancestor_record_offset":208928,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":209600,"parent_record_offset":209572,"ancestor_record_offset":208928,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":210260,"parent_record_offset":210196,"ancestor_record_offset":208928,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":210072,"parent_record_offset":209124,"ancestor_record_offset":208928,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":210284,"parent_record_offset":210260,"ancestor_record_offset":208928,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":210608,"parent_record_offset":209124,"ancestor_record_offset":208928,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":210324,"parent_record_offset":210144,"ancestor_record_offset":208928,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":209420,"parent_record_offset":209396,"ancestor_record_offset":208928,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":209740,"parent_record_offset":209716,"ancestor_record_offset":208928,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":210196,"parent_record_offset":210168,"ancestor_record_offset":208928,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":210232,"parent_record_offset":210196,"ancestor_record_offset":208928,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":210552,"parent_record_offset":210372,"ancestor_record_offset":208928,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":511,"record_offset":209796,"parent_record_offset":209776,"ancestor_record_offset":208928,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":209540,"parent_record_offset":209512,"ancestor_record_offset":208928,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":209572,"parent_record_offset":209540,"ancestor_record_offset":208928,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":209512,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":209372,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":209924,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":209456,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":209216,"parent_record_offset":209180,"ancestor_record_offset":208928,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":209776,"parent_record_offset":209672,"ancestor_record_offset":208928,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209716,"parent_record_offset":209692,"ancestor_record_offset":208928,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":210016,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209692,"parent_record_offset":209672,"ancestor_record_offset":208928,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209268,"parent_record_offset":209244,"ancestor_record_offset":208928,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209244,"parent_record_offset":209216,"ancestor_record_offset":208928,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209292,"parent_record_offset":209268,"ancestor_record_offset":208928,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209344,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":209672,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":511,"record_offset":209896,"parent_record_offset":209292,"ancestor_record_offset":208928,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00511.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/terrain-editing/source-ownership.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/terrain/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::terrain::impl$0::add_terrain_events
// 信息 ID: 847a1ad8dbfa8cd307343fa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41de8a -> 0x94ab30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::terrain::impl$0::add_terrain_systems
// 信息 ID: 025a1c803ed6d189d09dced6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41ded2 -> 0x94b0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::terrain::impl$0::insert_terrain_resources
// 信息 ID: 70ad1e603fb161af2f346d1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41d4b5 -> 0x94b330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
