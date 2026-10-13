// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/glade_themes/olden.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: eca6a0094bd442054ecf72c030318e6657c3b744a2f22c9c0c2f5ea7c6f8f7f2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: olden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1615
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::glade_themes::olden::startup_olden
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":45540,"rva":22362576,"va":5391071696}
// 编译实例 RVAs: 0x15539d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15991411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e88dfd0bff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391072910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409365d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12703409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e84f293eff","size":5,"soft_fail":false,"target_va":5378368976,"va":5391072380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0eeb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10768417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8dfaf5bff","size":5,"soft_fail":false,"target_va":5380304560,"va":5391072972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8244607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8813282ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5391072330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20775268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e864013d01","size":5,"soft_fail":false,"target_va":5411847808,"va":5391072535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: olden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20775615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8bf023d01","size":5,"soft_fail":false,"target_va":5411848144,"va":5391072524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14447327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8218d23ff","size":5,"soft_fail":false,"target_va":5376625536,"va":5391072858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20273104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8d0573501","size":5,"soft_fail":false,"target_va":5411347040,"va":5391073931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8419940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e89c857fff","size":5,"soft_fail":false,"target_va":5382655024,"va":5391074959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8648961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8ff067cff","size":5,"soft_fail":false,"target_va":5382424224,"va":5391073180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20467733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e815503801","size":5,"soft_fail":false,"target_va":5411541776,"va":5391074038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20468851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e873543801","size":5,"soft_fail":false,"target_va":5411542880,"va":5391074024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12698851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e81d3b3eff","size":5,"soft_fail":false,"target_va":5378374304,"va":5391073150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10719943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8396d5cff","size":5,"soft_fail":false,"target_va":5380353184,"va":5391073122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8328247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8c9eb80ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5391072322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9155768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391071696,"raw_hex":"e8484b74ff","size":5,"soft_fail":false,"target_va":5381917056,"va":5391072819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":46424,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":48544,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1615,"record_offset":45768,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":48508,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":47864,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1615,"record_offset":48436,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1615,"record_offset":46352,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":1615,"record_offset":46620,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":26275,"source_line":75,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::preplaced_autoclutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/preplaced_autoclutter.rs
//     evidence={"module":1615,"record_offset":45740,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":73987,"source_line":5,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":46068,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":26641,"source_line":269,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1615,"record_offset":47348,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1615,"record_offset":47644,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":46320,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":46180,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1615,"record_offset":46836,"parent_record_offset":45540,"ancestor_record_offset":45540,"inlinee":25133,"source_line":24,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1615,"record_offset":48072,"parent_record_offset":48048,"ancestor_record_offset":45540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1615,"record_offset":48048,"parent_record_offset":47960,"ancestor_record_offset":45540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":46500,"parent_record_offset":46468,"ancestor_record_offset":45540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":48728,"parent_record_offset":48700,"ancestor_record_offset":45540,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":48812,"parent_record_offset":48788,"ancestor_record_offset":45540,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":47988,"parent_record_offset":47960,"ancestor_record_offset":45540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":47960,"parent_record_offset":47936,"ancestor_record_offset":45540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":48788,"parent_record_offset":48756,"ancestor_record_offset":45540,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":46548,"parent_record_offset":46524,"ancestor_record_offset":45540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":46524,"parent_record_offset":46500,"ancestor_record_offset":45540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":48756,"parent_record_offset":48728,"ancestor_record_offset":45540,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":47936,"parent_record_offset":47912,"ancestor_record_offset":45540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":46468,"parent_record_offset":46424,"ancestor_record_offset":45540,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":48700,"parent_record_offset":48664,"ancestor_record_offset":45540,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":48632,"parent_record_offset":48604,"ancestor_record_offset":45540,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":48664,"parent_record_offset":48632,"ancestor_record_offset":45540,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":48576,"parent_record_offset":48544,"ancestor_record_offset":45540,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1615,"record_offset":48604,"parent_record_offset":48576,"ancestor_record_offset":45540,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1615,"record_offset":45884,"parent_record_offset":45792,"ancestor_record_offset":45540,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1615,"record_offset":47432,"parent_record_offset":47376,"ancestor_record_offset":45540,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1615,"record_offset":47084,"parent_record_offset":47032,"ancestor_record_offset":45540,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1615,"record_offset":47056,"parent_record_offset":47032,"ancestor_record_offset":45540,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1615,"record_offset":45852,"parent_record_offset":45828,"ancestor_record_offset":45540,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1615,"record_offset":48016,"parent_record_offset":47988,"ancestor_record_offset":45540,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1615,"record_offset":46796,"parent_record_offset":46772,"ancestor_record_offset":45540,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1615,"record_offset":45828,"parent_record_offset":45792,"ancestor_record_offset":45540,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1615,"record_offset":45792,"parent_record_offset":45768,"ancestor_record_offset":45540,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1615,"record_offset":45996,"parent_record_offset":45908,"ancestor_record_offset":45540,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":47888,"parent_record_offset":47864,"ancestor_record_offset":45540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1615,"record_offset":46388,"parent_record_offset":46352,"ancestor_record_offset":45540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":46020,"parent_record_offset":45996,"ancestor_record_offset":45540,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1615,"record_offset":46588,"parent_record_offset":46424,"ancestor_record_offset":45540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1615,"record_offset":45908,"parent_record_offset":45884,"ancestor_record_offset":45540,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":45964,"parent_record_offset":45940,"ancestor_record_offset":45540,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":45940,"parent_record_offset":45908,"ancestor_record_offset":45540,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":46116,"parent_record_offset":46092,"ancestor_record_offset":45540,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1615,"record_offset":46092,"parent_record_offset":46068,"ancestor_record_offset":45540,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1615,"record_offset":46684,"parent_record_offset":46656,"ancestor_record_offset":45540,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1615,"record_offset":46864,"parent_record_offset":46836,"ancestor_record_offset":45540,"inlinee":25134,"source_line":218,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1615,"record_offset":46900,"parent_record_offset":46864,"ancestor_record_offset":45540,"inlinee":25135,"source_line":184,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1615,"record_offset":46656,"parent_record_offset":46620,"ancestor_record_offset":45540,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":46944,"parent_record_offset":46900,"ancestor_record_offset":45540,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":47284,"parent_record_offset":46864,"ancestor_record_offset":45540,"inlinee":25143,"source_line":1811,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$19::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":47376,"parent_record_offset":47348,"ancestor_record_offset":45540,"inlinee":33088,"source_line":1121,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":47032,"parent_record_offset":46864,"ancestor_record_offset":45540,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1615,"record_offset":47732,"parent_record_offset":47700,"ancestor_record_offset":45540,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1615,"record_offset":47700,"parent_record_offset":47668,"ancestor_record_offset":45540,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1615,"record_offset":47668,"parent_record_offset":47644,"ancestor_record_offset":45540,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1615,"record_offset":46772,"parent_record_offset":46656,"ancestor_record_offset":45540,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1615,"record_offset":46732,"parent_record_offset":46708,"ancestor_record_offset":45540,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1615,"record_offset":46708,"parent_record_offset":46684,"ancestor_record_offset":45540,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1615,"record_offset":47764,"parent_record_offset":47732,"ancestor_record_offset":45540,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":46228,"parent_record_offset":46204,"ancestor_record_offset":45540,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":46276,"parent_record_offset":46252,"ancestor_record_offset":45540,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1615,"record_offset":46252,"parent_record_offset":46228,"ancestor_record_offset":45540,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":46144,"parent_record_offset":46092,"ancestor_record_offset":45540,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1615,"record_offset":46204,"parent_record_offset":46180,"ancestor_record_offset":45540,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01615.jsonl.gz"}
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
// 完整结构及原始类型记录: olden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: olden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: olden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::startup::glade_themes::olden::startup_olden
// 信息 ID: 454dad53d6eabc62ae43c568
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa78e64 -> 0x15539d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
