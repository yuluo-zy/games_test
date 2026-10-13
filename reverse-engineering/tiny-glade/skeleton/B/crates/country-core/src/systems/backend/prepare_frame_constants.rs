// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/backend/prepare_frame_constants.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f843d5159a3557ff63c0e0e217e15cc9588da102f7acd26a8416322f606ad3fc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: prepare_frame_constants.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 571, 580
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":571,"record_offset":138084,"rva":11485360,"va":5380194480}
// 编译实例 RVAs: 0xaf40b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2652961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8217b2800","size":5,"soft_fail":false,"target_va":5382847888,"va":5380194922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5515941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e85bd5abff","size":5,"soft_fail":false,"target_va":5374680032,"va":5380195968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5113332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e80cfab1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380194815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5116597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e84bedb1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380198096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31091066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e87a69da01","size":5,"soft_fail":false,"target_va":5411285568,"va":5380194497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31652074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8eaf8e201","size":5,"soft_fail":false,"target_va":5411847168,"va":5380195089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31650598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e826f3e201","size":5,"soft_fail":false,"target_va":5411847856,"va":5380197253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31650180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e884f1e201","size":5,"soft_fail":false,"target_va":5411847936,"va":5380197751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31650350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e82ef2e201","size":5,"soft_fail":false,"target_va":5411848016,"va":5380197661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31342742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e89640de01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380198216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31345351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8c74ade01","size":5,"soft_fail":false,"target_va":5411543584,"va":5380198228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::view_constants::ViewConstants::set_pixel_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t149216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8e0460200","size":5,"soft_fail":false,"target_va":5380344560,"va":5380195339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::view_constants::ViewportConstantBuilder::build
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t150520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8f84b0200","size":5,"soft_fail":false,"target_va":5380345696,"va":5380195171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::inverse
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8140213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380194480,"raw_hex":"e8b5357c00","size":5,"soft_fail":false,"target_va":5388336384,"va":5380196166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":571,"record_offset":138932,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":571,"record_offset":139160,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":143724,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":139992,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":143852,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":571,"record_offset":138988,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":32671,"source_line":32,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::ColorGradingState::flattened
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":571,"record_offset":138336,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":28414,"source_line":50,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":571,"record_offset":138308,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":27146,"source_line":94,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::GladeType::theme_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":571,"record_offset":142176,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":31607,"source_line":65,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/prepare_frame_constants.rs
//     evidence={"module":571,"record_offset":141644,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":32674,"source_line":111,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":571,"record_offset":139964,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":27049,"source_line":432,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":571,"record_offset":142232,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":30378,"source_line":162,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":571,"record_offset":142988,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":13671,"source_line":1193,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":571,"record_offset":142372,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":571,"record_offset":142204,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":7676,"source_line":421,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":571,"record_offset":143752,"parent_record_offset":138084,"ancestor_record_offset":138084,"inlinee":7675,"source_line":502,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":138732,"parent_record_offset":138708,"ancestor_record_offset":138084,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":138756,"parent_record_offset":138732,"ancestor_record_offset":138084,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":144052,"parent_record_offset":144028,"ancestor_record_offset":138084,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":138708,"parent_record_offset":138680,"ancestor_record_offset":138084,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":144028,"parent_record_offset":143972,"ancestor_record_offset":138084,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":138620,"parent_record_offset":138592,"ancestor_record_offset":138084,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":144000,"parent_record_offset":143972,"ancestor_record_offset":138084,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":143972,"parent_record_offset":143948,"ancestor_record_offset":138084,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":138680,"parent_record_offset":138648,"ancestor_record_offset":138084,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":138648,"parent_record_offset":138620,"ancestor_record_offset":138084,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":143948,"parent_record_offset":143924,"ancestor_record_offset":138084,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":571,"record_offset":138556,"parent_record_offset":138528,"ancestor_record_offset":138084,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":571,"record_offset":138528,"parent_record_offset":138500,"ancestor_record_offset":138084,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":571,"record_offset":138472,"parent_record_offset":138444,"ancestor_record_offset":138084,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":571,"record_offset":138500,"parent_record_offset":138472,"ancestor_record_offset":138084,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":571,"record_offset":138444,"parent_record_offset":138416,"ancestor_record_offset":138084,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":571,"record_offset":138416,"parent_record_offset":138380,"ancestor_record_offset":138084,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":139560,"parent_record_offset":139536,"ancestor_record_offset":138084,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":138592,"parent_record_offset":138556,"ancestor_record_offset":138084,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":139536,"parent_record_offset":139284,"ancestor_record_offset":138084,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":571,"record_offset":138956,"parent_record_offset":138932,"ancestor_record_offset":138084,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":571,"record_offset":139248,"parent_record_offset":139196,"ancestor_record_offset":138084,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":571,"record_offset":139284,"parent_record_offset":139248,"ancestor_record_offset":138084,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":571,"record_offset":141276,"parent_record_offset":140016,"ancestor_record_offset":138084,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":571,"record_offset":141304,"parent_record_offset":141276,"ancestor_record_offset":138084,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":571,"record_offset":139196,"parent_record_offset":139160,"ancestor_record_offset":138084,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":140880,"parent_record_offset":140856,"ancestor_record_offset":138084,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":571,"record_offset":140388,"parent_record_offset":140364,"ancestor_record_offset":138084,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":571,"record_offset":140472,"parent_record_offset":140448,"ancestor_record_offset":138084,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":138832,"parent_record_offset":138808,"ancestor_record_offset":138084,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":140976,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":571,"record_offset":141372,"parent_record_offset":141344,"ancestor_record_offset":138084,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":571,"record_offset":141344,"parent_record_offset":141304,"ancestor_record_offset":138084,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":571,"record_offset":141452,"parent_record_offset":141428,"ancestor_record_offset":138084,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":140060,"parent_record_offset":140016,"ancestor_record_offset":138084,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":140016,"parent_record_offset":139992,"ancestor_record_offset":138084,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":139488,"parent_record_offset":139456,"ancestor_record_offset":138084,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":139336,"parent_record_offset":139284,"ancestor_record_offset":138084,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":571,"record_offset":140620,"parent_record_offset":140592,"ancestor_record_offset":138084,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":571,"record_offset":140592,"parent_record_offset":140564,"ancestor_record_offset":138084,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":571,"record_offset":142404,"parent_record_offset":142372,"ancestor_record_offset":138084,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":571,"record_offset":141396,"parent_record_offset":141372,"ancestor_record_offset":138084,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":571,"record_offset":141236,"parent_record_offset":140128,"ancestor_record_offset":138084,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":571,"record_offset":141428,"parent_record_offset":141344,"ancestor_record_offset":138084,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":571,"record_offset":138808,"parent_record_offset":138556,"ancestor_record_offset":138084,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":143876,"parent_record_offset":143852,"ancestor_record_offset":138084,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":139856,"parent_record_offset":139832,"ancestor_record_offset":138084,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":140740,"parent_record_offset":140716,"ancestor_record_offset":138084,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":141576,"parent_record_offset":141552,"ancestor_record_offset":138084,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":139392,"parent_record_offset":139284,"ancestor_record_offset":138084,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":139808,"parent_record_offset":139284,"ancestor_record_offset":138084,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":139456,"parent_record_offset":139424,"ancestor_record_offset":138084,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":139424,"parent_record_offset":139392,"ancestor_record_offset":138084,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":139832,"parent_record_offset":139808,"ancestor_record_offset":138084,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":141552,"parent_record_offset":141528,"ancestor_record_offset":138084,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":141504,"parent_record_offset":141452,"ancestor_record_offset":138084,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":141528,"parent_record_offset":141504,"ancestor_record_offset":138084,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::directional_light::DirectionalLight::view_to_clip
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/directional_light.rs
//     evidence={"module":571,"record_offset":141696,"parent_record_offset":141644,"ancestor_record_offset":138084,"inlinee":32676,"source_line":35,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":571,"record_offset":141780,"parent_record_offset":141644,"ancestor_record_offset":138084,"inlinee":32671,"source_line":32,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":571,"record_offset":138380,"parent_record_offset":138336,"ancestor_record_offset":138084,"inlinee":27119,"source_line":36,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":571,"record_offset":138892,"parent_record_offset":138336,"ancestor_record_offset":138084,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":571,"record_offset":140128,"parent_record_offset":140092,"ancestor_record_offset":138084,"inlinee":28320,"source_line":89,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":571,"record_offset":140904,"parent_record_offset":140880,"ancestor_record_offset":138084,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":571,"record_offset":140928,"parent_record_offset":140904,"ancestor_record_offset":138084,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/prepare_frame_constants.rs
//     evidence={"module":571,"record_offset":140092,"parent_record_offset":140060,"ancestor_record_offset":138084,"inlinee":32673,"source_line":165,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":571,"record_offset":141744,"parent_record_offset":141720,"ancestor_record_offset":138084,"inlinee":25144,"source_line":106,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::orthographic_rh
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":571,"record_offset":141720,"parent_record_offset":141696,"ancestor_record_offset":138084,"inlinee":32677,"source_line":1096,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":571,"record_offset":142256,"parent_record_offset":142232,"ancestor_record_offset":138084,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140532,"parent_record_offset":140504,"ancestor_record_offset":138084,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140564,"parent_record_offset":140532,"ancestor_record_offset":138084,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140504,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":140364,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":141004,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":140448,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":571,"record_offset":141156,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":571,"record_offset":140856,"parent_record_offset":140832,"ancestor_record_offset":138084,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140184,"parent_record_offset":140156,"ancestor_record_offset":138084,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140212,"parent_record_offset":140184,"ancestor_record_offset":138084,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140832,"parent_record_offset":140664,"ancestor_record_offset":138084,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":141196,"parent_record_offset":140212,"ancestor_record_offset":138084,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140772,"parent_record_offset":140688,"ancestor_record_offset":138084,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140716,"parent_record_offset":140688,"ancestor_record_offset":138084,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":141096,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140688,"parent_record_offset":140664,"ancestor_record_offset":138084,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140276,"parent_record_offset":140248,"ancestor_record_offset":138084,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140248,"parent_record_offset":140212,"ancestor_record_offset":138084,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140304,"parent_record_offset":140276,"ancestor_record_offset":138084,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":141128,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140664,"parent_record_offset":140304,"ancestor_record_offset":138084,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":571,"record_offset":140156,"parent_record_offset":140128,"ancestor_record_offset":138084,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FA2
// 首条位置: {"module":571,"inlinee":32674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::directional_light::DirectionalLight::view_to_clip
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/directional_light.rs
//     evidence={"module":571,"record_offset":141696,"parent_record_offset":141644,"ancestor_record_offset":141644,"inlinee":32676,"source_line":35,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":571,"record_offset":141780,"parent_record_offset":141644,"ancestor_record_offset":141644,"inlinee":32671,"source_line":32,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":571,"record_offset":141744,"parent_record_offset":141720,"ancestor_record_offset":141644,"inlinee":25144,"source_line":106,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::orthographic_rh
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":571,"record_offset":141720,"parent_record_offset":141696,"ancestor_record_offset":141644,"inlinee":32677,"source_line":1096,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FA1
// 首条位置: {"module":571,"inlinee":32673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":571,"record_offset":140128,"parent_record_offset":140092,"ancestor_record_offset":140092,"inlinee":28320,"source_line":89,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":140880,"parent_record_offset":140856,"ancestor_record_offset":140092,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":571,"record_offset":140388,"parent_record_offset":140364,"ancestor_record_offset":140092,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":571,"record_offset":140472,"parent_record_offset":140448,"ancestor_record_offset":140092,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":140412,"parent_record_offset":140388,"ancestor_record_offset":140092,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":140976,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":571,"record_offset":140620,"parent_record_offset":140592,"ancestor_record_offset":140092,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":571,"record_offset":140592,"parent_record_offset":140564,"ancestor_record_offset":140092,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":571,"record_offset":141236,"parent_record_offset":140128,"ancestor_record_offset":140092,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":140740,"parent_record_offset":140716,"ancestor_record_offset":140092,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":571,"record_offset":140904,"parent_record_offset":140880,"ancestor_record_offset":140092,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":571,"record_offset":140928,"parent_record_offset":140904,"ancestor_record_offset":140092,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140532,"parent_record_offset":140504,"ancestor_record_offset":140092,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140564,"parent_record_offset":140532,"ancestor_record_offset":140092,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":571,"record_offset":140504,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":140364,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":141004,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":571,"record_offset":140448,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":571,"record_offset":141156,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":571,"record_offset":140856,"parent_record_offset":140832,"ancestor_record_offset":140092,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140184,"parent_record_offset":140156,"ancestor_record_offset":140092,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140212,"parent_record_offset":140184,"ancestor_record_offset":140092,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":140832,"parent_record_offset":140664,"ancestor_record_offset":140092,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":571,"record_offset":141196,"parent_record_offset":140212,"ancestor_record_offset":140092,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140772,"parent_record_offset":140688,"ancestor_record_offset":140092,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140716,"parent_record_offset":140688,"ancestor_record_offset":140092,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":141096,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140688,"parent_record_offset":140664,"ancestor_record_offset":140092,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140276,"parent_record_offset":140248,"ancestor_record_offset":140092,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140248,"parent_record_offset":140212,"ancestor_record_offset":140092,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140304,"parent_record_offset":140276,"ancestor_record_offset":140092,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":141128,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":571,"record_offset":140664,"parent_record_offset":140304,"ancestor_record_offset":140092,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":571,"record_offset":140156,"parent_record_offset":140128,"ancestor_record_offset":140092,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00571.jsonl.gz"}
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
// 完整结构及原始类型记录: prepare_frame_constants.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: prepare_frame_constants.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: prepare_frame_constants.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::tree_audio::TreeAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::TreeAmbience>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >", "bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>", "bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants
// 信息 ID: 207c34baa7632afa07273266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 420c56ef159abc531764f798: void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>)> > @ 0x3c6fa6 -> 0xaf40b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>)> > > @ 0x464893 -> 0xaf40b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
// 信息 ID: e483b0a095b5d12a2786839d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::prepare_frame_constants::prepare_frame_constants", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=141644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=141812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=141980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$1
// 信息 ID: 95bbeaa9b0dff2484896634f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::backend::prepare_frame_constants::prepare_frame_constants", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=571, inline_record=140092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
