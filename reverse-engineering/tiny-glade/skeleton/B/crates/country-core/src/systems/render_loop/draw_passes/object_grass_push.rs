// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 154bf46f36708f3432c7b42eccfae6dd6bf33c77be381af790004a4611c01c31
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: object_grass_push.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 532, 1585
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":63348,"rva":10332832,"va":5379041952}
// 编译实例 RVAs: 0x9daaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9443521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e83fe76fff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379044316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=8
//     target_aliases=19 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9426516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8ac2970ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379042623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0be0
//     owner=unresolved_namespace; count=2
//     target_aliases=251 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4237034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e81659bfff","size":5,"soft_fail":false,"target_va":5374807008,"va":5379044037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3963261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e88386c3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379044760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1348938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8b66aebff","size":5,"soft_fail":false,"target_va":5377696704,"va":5379045637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1344573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8c37bebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379045880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-457136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e85006f9ff","size":5,"soft_fail":false,"target_va":5378587424,"va":5379044555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2143096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e878b32000","size":5,"soft_fail":false,"target_va":5381187408,"va":5379044307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fde80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9579465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8c92b9200","size":5,"soft_fail":false,"target_va":5388623488,"va":5379044018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141657e90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13094492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e85ccec700","size":5,"soft_fail":false,"target_va":5392137872,"va":5379043375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32625054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e89ed1f101","size":5,"soft_fail":false,"target_va":5411669968,"va":5379044909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32802576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e81087f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379044587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13104414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e81ef5c700","size":5,"soft_fail":false,"target_va":5392147776,"va":5379043357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::validate_world::panic_mismatched
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32455560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8883bef01","size":5,"soft_fail":false,"target_va":5411500544,"va":5379044979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32496612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8e4dbef01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379044967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t32496761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e879dcef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379045010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-26665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8d797ffff","size":5,"soft_fail":false,"target_va":5379015376,"va":5379042036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32501249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e801eeef01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379042106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-668421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8fbccf5ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5379042720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-647026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e88e20f6ff","size":5,"soft_fail":false,"target_va":5378395952,"va":5379042973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-871116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e834b5f2ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5379044695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-874680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e848a7f2ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5379044499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1522155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e815c6e8ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5379044182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1520206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8b2cde8ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5379044793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2342969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e839c02300","size":5,"soft_fail":false,"target_va":5381385536,"va":5379042562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2348158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e87ed42300","size":5,"soft_fail":false,"target_va":5381390640,"va":5379042477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2341615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8efba2300","size":5,"soft_fail":false,"target_va":5381386480,"va":5379044860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2342692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e824bf2300","size":5,"soft_fail":false,"target_va":5381384704,"va":5379042007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2073779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8b3a41f00","size":5,"soft_fail":false,"target_va":5381117840,"va":5379044056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9779990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8163b9500","size":5,"soft_fail":false,"target_va":5388825296,"va":5379045301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32492144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e870caef01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379044955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2866128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379041952,"raw_hex":"e8d0bb2b00","size":5,"soft_fail":false,"target_va":5381909824,"va":5379043691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":74108,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":532,"record_offset":74164,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":532,"record_offset":70300,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":30255,"source_line":1185,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":532,"record_offset":70244,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":30253,"source_line":1535,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":66020,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":532,"record_offset":70988,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":532,"record_offset":73788,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":532,"record_offset":73844,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":532,"record_offset":70932,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":66808,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":532,"record_offset":75020,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":74776,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":532,"record_offset":74304,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":532,"record_offset":73928,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":68572,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":67620,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":65036,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":63604,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":64060,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":74932,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":74848,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":532,"record_offset":75276,"parent_record_offset":75252,"ancestor_record_offset":63348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":532,"record_offset":75252,"parent_record_offset":75164,"ancestor_record_offset":63348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":66456,"parent_record_offset":66432,"ancestor_record_offset":63348,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":75192,"parent_record_offset":75164,"ancestor_record_offset":63348,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":75164,"parent_record_offset":75140,"ancestor_record_offset":63348,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":66504,"parent_record_offset":66480,"ancestor_record_offset":63348,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":66480,"parent_record_offset":66456,"ancestor_record_offset":63348,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":75140,"parent_record_offset":75116,"ancestor_record_offset":63348,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":66432,"parent_record_offset":66408,"ancestor_record_offset":63348,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":66408,"parent_record_offset":66368,"ancestor_record_offset":63348,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":73868,"parent_record_offset":73844,"ancestor_record_offset":63348,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":66368,"parent_record_offset":66344,"ancestor_record_offset":63348,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":532,"record_offset":75044,"parent_record_offset":75020,"ancestor_record_offset":63348,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":532,"record_offset":75092,"parent_record_offset":75068,"ancestor_record_offset":63348,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":532,"record_offset":70696,"parent_record_offset":70672,"ancestor_record_offset":63348,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":532,"record_offset":70720,"parent_record_offset":70696,"ancestor_record_offset":63348,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":532,"record_offset":70328,"parent_record_offset":70300,"ancestor_record_offset":63348,"inlinee":30256,"source_line":326,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":532,"record_offset":70388,"parent_record_offset":70328,"ancestor_record_offset":63348,"inlinee":30258,"source_line":342,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_unchecked_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":532,"record_offset":70524,"parent_record_offset":70388,"ancestor_record_offset":63348,"inlinee":30259,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":532,"record_offset":70360,"parent_record_offset":70328,"ancestor_record_offset":63348,"inlinee":30257,"source_line":512,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":66756,"parent_record_offset":66732,"ancestor_record_offset":63348,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":66732,"parent_record_offset":66708,"ancestor_record_offset":63348,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":66680,"parent_record_offset":66648,"ancestor_record_offset":63348,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":66624,"parent_record_offset":66088,"ancestor_record_offset":63348,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":68244,"parent_record_offset":67704,"ancestor_record_offset":63348,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":66648,"parent_record_offset":66624,"ancestor_record_offset":63348,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":66116,"parent_record_offset":66088,"ancestor_record_offset":63348,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":66708,"parent_record_offset":66648,"ancestor_record_offset":63348,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":68328,"parent_record_offset":68268,"ancestor_record_offset":63348,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":66164,"parent_record_offset":66140,"ancestor_record_offset":63348,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":66140,"parent_record_offset":66116,"ancestor_record_offset":63348,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":66344,"parent_record_offset":66320,"ancestor_record_offset":63348,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":532,"record_offset":70672,"parent_record_offset":70300,"ancestor_record_offset":63348,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":532,"record_offset":70424,"parent_record_offset":70388,"ancestor_record_offset":63348,"inlinee":4113,"source_line":187,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":532,"record_offset":70268,"parent_record_offset":70244,"ancestor_record_offset":63348,"inlinee":30254,"source_line":1559,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":532,"record_offset":68444,"parent_record_offset":68420,"ancestor_record_offset":63348,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::identifier::impl$9::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/identifier.rs
//     evidence={"module":532,"record_offset":70488,"parent_record_offset":70464,"ancestor_record_offset":63348,"inlinee":4117,"source_line":11,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":68420,"parent_record_offset":67704,"ancestor_record_offset":63348,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":66048,"parent_record_offset":66020,"ancestor_record_offset":63348,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":68600,"parent_record_offset":68572,"ancestor_record_offset":63348,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":66088,"parent_record_offset":66020,"ancestor_record_offset":63348,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":67648,"parent_record_offset":67620,"ancestor_record_offset":63348,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":67704,"parent_record_offset":67648,"ancestor_record_offset":63348,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65372,"parent_record_offset":65344,"ancestor_record_offset":63348,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":74604,"parent_record_offset":74576,"ancestor_record_offset":63348,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65424,"parent_record_offset":65400,"ancestor_record_offset":63348,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65400,"parent_record_offset":65372,"ancestor_record_offset":63348,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65320,"parent_record_offset":65292,"ancestor_record_offset":63348,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65344,"parent_record_offset":65320,"ancestor_record_offset":63348,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65516,"parent_record_offset":65344,"ancestor_record_offset":63348,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":532,"record_offset":65628,"parent_record_offset":65604,"ancestor_record_offset":63348,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":532,"record_offset":70464,"parent_record_offset":70424,"ancestor_record_offset":63348,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":532,"record_offset":73960,"parent_record_offset":73928,"ancestor_record_offset":63348,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":532,"record_offset":73812,"parent_record_offset":73788,"ancestor_record_offset":63348,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":532,"record_offset":70956,"parent_record_offset":70932,"ancestor_record_offset":63348,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":532,"record_offset":73892,"parent_record_offset":73868,"ancestor_record_offset":63348,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":532,"record_offset":65448,"parent_record_offset":65424,"ancestor_record_offset":63348,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":532,"record_offset":75220,"parent_record_offset":75192,"ancestor_record_offset":63348,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":532,"record_offset":63760,"parent_record_offset":63728,"ancestor_record_offset":63348,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":65188,"parent_record_offset":65144,"ancestor_record_offset":63348,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":74816,"parent_record_offset":74776,"ancestor_record_offset":63348,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":66280,"parent_record_offset":66256,"ancestor_record_offset":63348,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":66320,"parent_record_offset":66140,"ancestor_record_offset":63348,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":68540,"parent_record_offset":67620,"ancestor_record_offset":63348,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":73688,"parent_record_offset":73664,"ancestor_record_offset":63348,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":66872,"parent_record_offset":66844,"ancestor_record_offset":63348,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":532,"record_offset":64008,"parent_record_offset":63980,"ancestor_record_offset":63348,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":532,"record_offset":65976,"parent_record_offset":65144,"ancestor_record_offset":63348,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":532,"record_offset":65580,"parent_record_offset":65292,"ancestor_record_offset":63348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":532,"record_offset":74132,"parent_record_offset":74108,"ancestor_record_offset":63348,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":532,"record_offset":64472,"parent_record_offset":64448,"ancestor_record_offset":63348,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":532,"record_offset":74244,"parent_record_offset":74164,"ancestor_record_offset":63348,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":532,"record_offset":64448,"parent_record_offset":64424,"ancestor_record_offset":63348,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":532,"record_offset":64656,"parent_record_offset":64628,"ancestor_record_offset":63348,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":532,"record_offset":74268,"parent_record_offset":74244,"ancestor_record_offset":63348,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":532,"record_offset":65472,"parent_record_offset":65448,"ancestor_record_offset":63348,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":66192,"parent_record_offset":66164,"ancestor_record_offset":63348,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":66572,"parent_record_offset":66548,"ancestor_record_offset":63348,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":66228,"parent_record_offset":66192,"ancestor_record_offset":63348,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":66548,"parent_record_offset":66368,"ancestor_record_offset":63348,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":64400,"parent_record_offset":64376,"ancestor_record_offset":63348,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":64424,"parent_record_offset":64400,"ancestor_record_offset":63348,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":64728,"parent_record_offset":64700,"ancestor_record_offset":63348,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":532,"record_offset":64376,"parent_record_offset":64260,"ancestor_record_offset":63348,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":532,"record_offset":63728,"parent_record_offset":63700,"ancestor_record_offset":63348,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":532,"record_offset":64088,"parent_record_offset":64060,"ancestor_record_offset":63348,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":532,"record_offset":64228,"parent_record_offset":64196,"ancestor_record_offset":63348,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":532,"record_offset":64196,"parent_record_offset":64160,"ancestor_record_offset":63348,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":532,"record_offset":64136,"parent_record_offset":64112,"ancestor_record_offset":63348,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":532,"record_offset":64160,"parent_record_offset":64136,"ancestor_record_offset":63348,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":532,"record_offset":64112,"parent_record_offset":64088,"ancestor_record_offset":63348,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":532,"record_offset":64700,"parent_record_offset":64160,"ancestor_record_offset":63348,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63700,"parent_record_offset":63668,"ancestor_record_offset":63348,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63668,"parent_record_offset":63640,"ancestor_record_offset":63348,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63860,"parent_record_offset":63828,"ancestor_record_offset":63348,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63956,"parent_record_offset":63860,"ancestor_record_offset":63348,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63828,"parent_record_offset":63804,"ancestor_record_offset":63348,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63640,"parent_record_offset":63604,"ancestor_record_offset":63348,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":63980,"parent_record_offset":63956,"ancestor_record_offset":63348,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":532,"record_offset":64260,"parent_record_offset":64228,"ancestor_record_offset":63348,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":532,"record_offset":64544,"parent_record_offset":64260,"ancestor_record_offset":63348,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":532,"record_offset":64516,"parent_record_offset":64260,"ancestor_record_offset":63348,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":532,"record_offset":64320,"parent_record_offset":64260,"ancestor_record_offset":63348,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":68468,"parent_record_offset":68444,"ancestor_record_offset":63348,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":68492,"parent_record_offset":68468,"ancestor_record_offset":63348,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":532,"record_offset":74352,"parent_record_offset":74304,"ancestor_record_offset":63348,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":532,"record_offset":74036,"parent_record_offset":74008,"ancestor_record_offset":63348,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":532,"record_offset":74008,"parent_record_offset":73984,"ancestor_record_offset":63348,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":532,"record_offset":73984,"parent_record_offset":73960,"ancestor_record_offset":63348,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":532,"record_offset":73664,"parent_record_offset":71024,"ancestor_record_offset":63348,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":532,"record_offset":72744,"parent_record_offset":71024,"ancestor_record_offset":63348,"inlinee":28514,"source_line":704,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
//     evidence={"module":532,"record_offset":71024,"parent_record_offset":70988,"ancestor_record_offset":63348,"inlinee":30387,"source_line":66,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":71072,"parent_record_offset":71024,"ancestor_record_offset":63348,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":532,"record_offset":72796,"parent_record_offset":72772,"ancestor_record_offset":63348,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":532,"record_offset":72772,"parent_record_offset":71024,"ancestor_record_offset":63348,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":532,"record_offset":74896,"parent_record_offset":74872,"ancestor_record_offset":63348,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":65072,"parent_record_offset":65036,"ancestor_record_offset":63348,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":75400,"parent_record_offset":75368,"ancestor_record_offset":63348,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":75900,"parent_record_offset":75640,"ancestor_record_offset":63348,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":532,"record_offset":65216,"parent_record_offset":65188,"ancestor_record_offset":63348,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":65292,"parent_record_offset":65144,"ancestor_record_offset":63348,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":532,"record_offset":65552,"parent_record_offset":65292,"ancestor_record_offset":63348,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":74956,"parent_record_offset":74932,"ancestor_record_offset":63348,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":74872,"parent_record_offset":74848,"ancestor_record_offset":63348,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":532,"record_offset":74060,"parent_record_offset":74036,"ancestor_record_offset":63348,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":532,"record_offset":74384,"parent_record_offset":74352,"ancestor_record_offset":63348,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":532,"record_offset":74448,"parent_record_offset":74416,"ancestor_record_offset":63348,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":532,"record_offset":74576,"parent_record_offset":74516,"ancestor_record_offset":63348,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":532,"record_offset":74416,"parent_record_offset":74384,"ancestor_record_offset":63348,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":532,"record_offset":65244,"parent_record_offset":65216,"ancestor_record_offset":63348,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":532,"record_offset":65144,"parent_record_offset":65108,"ancestor_record_offset":63348,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":532,"record_offset":65108,"parent_record_offset":65072,"ancestor_record_offset":63348,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76A9
// 首条位置: {"module":532,"inlinee":30377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":23668,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1585,"record_offset":23716,"parent_record_offset":23692,"ancestor_record_offset":23668,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B3
// 首条位置: {"module":532,"inlinee":30387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":532,"record_offset":73664,"parent_record_offset":71024,"ancestor_record_offset":71024,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":532,"record_offset":72744,"parent_record_offset":71024,"ancestor_record_offset":71024,"inlinee":28514,"source_line":704,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":71072,"parent_record_offset":71024,"ancestor_record_offset":71024,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":532,"record_offset":72772,"parent_record_offset":71024,"ancestor_record_offset":71024,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":71524,"parent_record_offset":71500,"ancestor_record_offset":71024,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":71572,"parent_record_offset":71548,"ancestor_record_offset":71024,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":71548,"parent_record_offset":71524,"ancestor_record_offset":71024,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":71500,"parent_record_offset":71476,"ancestor_record_offset":71024,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":71476,"parent_record_offset":71436,"ancestor_record_offset":71024,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":71436,"parent_record_offset":71412,"ancestor_record_offset":71024,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":71824,"parent_record_offset":71800,"ancestor_record_offset":71024,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":71800,"parent_record_offset":71776,"ancestor_record_offset":71024,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":71748,"parent_record_offset":71716,"ancestor_record_offset":71024,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":71692,"parent_record_offset":71156,"ancestor_record_offset":71024,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":71716,"parent_record_offset":71692,"ancestor_record_offset":71024,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":71184,"parent_record_offset":71156,"ancestor_record_offset":71024,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":71776,"parent_record_offset":71716,"ancestor_record_offset":71024,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":71232,"parent_record_offset":71208,"ancestor_record_offset":71024,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":71208,"parent_record_offset":71184,"ancestor_record_offset":71024,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":71412,"parent_record_offset":71388,"ancestor_record_offset":71024,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":71100,"parent_record_offset":71072,"ancestor_record_offset":71024,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":71156,"parent_record_offset":71100,"ancestor_record_offset":71024,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":71324,"parent_record_offset":71260,"ancestor_record_offset":71024,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":71348,"parent_record_offset":71324,"ancestor_record_offset":71024,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":71388,"parent_record_offset":71208,"ancestor_record_offset":71024,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":71876,"parent_record_offset":71072,"ancestor_record_offset":71024,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":73688,"parent_record_offset":73664,"ancestor_record_offset":71024,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":71128,"parent_record_offset":71100,"ancestor_record_offset":71024,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":71260,"parent_record_offset":71232,"ancestor_record_offset":71024,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":71640,"parent_record_offset":71616,"ancestor_record_offset":71024,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":71296,"parent_record_offset":71260,"ancestor_record_offset":71024,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":71616,"parent_record_offset":71436,"ancestor_record_offset":71024,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":532,"record_offset":72796,"parent_record_offset":72772,"ancestor_record_offset":71024,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
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
// 完整结构及原始类型记录: object_grass_push.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: object_grass_push.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: object_grass_push.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":74776,"parent_record_offset":63348,"ancestor_record_offset":63348,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161795,"position_among_mapped_render_targets":14,"mapped_render_target_count":59,"previous_callsite":5161784,"next_callsite":5161806,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: object_grass_push.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
// 信息 ID: 307e14a950eea5268b668b42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec343 -> 0x9daaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$0
// 信息 ID: 8d5a30f21b5e857cb1e7e77e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0 <- core::iter::adapters::chain::impl$1::next<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::next<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> >; module=1585, inline_record=23668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
// 信息 ID: 8a855129ab76f2dbb0b7f653
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=532, inline_record=71024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
