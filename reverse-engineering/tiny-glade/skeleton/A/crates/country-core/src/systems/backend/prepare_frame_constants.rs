// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/backend/prepare_frame_constants.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 76341f594d2875aed3d4e1c4ccf1e16bc42652dbff0b45bf5313458a29f35297
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: prepare_frame_constants.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 392
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":392,"record_offset":22484,"rva":5359824,"va":5374068944}
// 编译实例 RVAs: 0x51c8d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4436730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8064dbcff","size":5,"soft_fail":false,"target_va":5369632448,"va":5374069173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1319428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8fcddebff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374069119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1321865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e877d4ebff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374071572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28831146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8aaedb701","size":5,"soft_fail":false,"target_va":5402900112,"va":5374068961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29523945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8e97fc201","size":5,"soft_fail":false,"target_va":5403593360,"va":5374069410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29523227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e81b7dc201","size":5,"soft_fail":false,"target_va":5403594048,"va":5374070816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prepare_frame_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29522882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8c27bc201","size":5,"soft_fail":false,"target_va":5403594128,"va":5374071241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28906428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8bc13b901","size":5,"soft_fail":false,"target_va":5402978144,"va":5374071711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28907453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8bd17b901","size":5,"soft_fail":false,"target_va":5402979152,"va":5374071694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::view_constants::ViewConstants::set_pixel_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t414389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e8b5520600","size":5,"soft_fail":false,"target_va":5374484000,"va":5374069606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::view_constants::ViewportConstantBuilder::build
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t415541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e835570600","size":5,"soft_fail":false,"target_va":5374485040,"va":5374069494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::mat4::Mat4::inverse
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10314140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374068944,"raw_hex":"e89c619d00","size":5,"soft_fail":false,"target_va":5384383904,"va":5374069759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":392,"record_offset":23360,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":25676,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":392,"record_offset":23420,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22351,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::ColorGradingState::flattened
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":392,"record_offset":22720,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22171,"source_line":49,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":392,"record_offset":22692,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22349,"source_line":76,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::GladeType::theme_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":392,"record_offset":24056,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22358,"source_line":56,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/prepare_frame_constants.rs
//     evidence={"module":392,"record_offset":23536,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22353,"source_line":80,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":392,"record_offset":24028,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":21922,"source_line":429,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":24112,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":24876,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":22360,"source_line":1027,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":24252,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":24084,"parent_record_offset":22484,"ancestor_record_offset":22484,"inlinee":6462,"source_line":491,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":23104,"parent_record_offset":23080,"ancestor_record_offset":22484,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":23128,"parent_record_offset":23104,"ancestor_record_offset":22484,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":22484,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":23080,"parent_record_offset":23052,"ancestor_record_offset":22484,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":25828,"parent_record_offset":25772,"ancestor_record_offset":22484,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":25800,"parent_record_offset":25772,"ancestor_record_offset":22484,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":23052,"parent_record_offset":23020,"ancestor_record_offset":22484,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":23020,"parent_record_offset":22988,"ancestor_record_offset":22484,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":23200,"parent_record_offset":23020,"ancestor_record_offset":22484,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":25772,"parent_record_offset":25748,"ancestor_record_offset":22484,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":392,"record_offset":22952,"parent_record_offset":22920,"ancestor_record_offset":22484,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":392,"record_offset":22920,"parent_record_offset":22888,"ancestor_record_offset":22484,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":392,"record_offset":22856,"parent_record_offset":22824,"ancestor_record_offset":22484,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":392,"record_offset":22888,"parent_record_offset":22856,"ancestor_record_offset":22484,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":392,"record_offset":22824,"parent_record_offset":22792,"ancestor_record_offset":22484,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":392,"record_offset":22792,"parent_record_offset":22760,"ancestor_record_offset":22484,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":22988,"parent_record_offset":22952,"ancestor_record_offset":22484,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":392,"record_offset":23388,"parent_record_offset":23360,"ancestor_record_offset":22484,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":392,"record_offset":23260,"parent_record_offset":23236,"ancestor_record_offset":22484,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":24284,"parent_record_offset":24252,"ancestor_record_offset":22484,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":23236,"parent_record_offset":22952,"ancestor_record_offset":22484,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":25700,"parent_record_offset":25676,"ancestor_record_offset":22484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":23156,"parent_record_offset":23128,"ancestor_record_offset":22484,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::directional_light::DirectionalLight::view_to_clip
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/directional_light.rs
//     evidence={"module":392,"record_offset":23584,"parent_record_offset":23536,"ancestor_record_offset":22484,"inlinee":22355,"source_line":34,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":392,"record_offset":23668,"parent_record_offset":23536,"ancestor_record_offset":22484,"inlinee":22351,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":392,"record_offset":22760,"parent_record_offset":22720,"ancestor_record_offset":22484,"inlinee":22117,"source_line":35,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":392,"record_offset":23320,"parent_record_offset":22720,"ancestor_record_offset":22484,"inlinee":22119,"source_line":24,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":392,"record_offset":23632,"parent_record_offset":23608,"ancestor_record_offset":22484,"inlinee":22357,"source_line":103,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::orthographic_rh
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":392,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":22484,"inlinee":22356,"source_line":1043,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":392,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":22484,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5751
// 首条位置: {"module":392,"inlinee":22353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::directional_light::DirectionalLight::view_to_clip
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/directional_light.rs
//     evidence={"module":392,"record_offset":23584,"parent_record_offset":23536,"ancestor_record_offset":23536,"inlinee":22355,"source_line":34,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::view_constants::ViewConstants::builder
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/view_constants.rs
//     evidence={"module":392,"record_offset":23668,"parent_record_offset":23536,"ancestor_record_offset":23536,"inlinee":22351,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":392,"record_offset":23632,"parent_record_offset":23608,"ancestor_record_offset":23536,"inlinee":22357,"source_line":103,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::orthographic_rh
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":392,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":23536,"inlinee":22356,"source_line":1043,"archive":"_pdb/A/modules/00392.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>", "bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>", "bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants
// 信息 ID: 5415663c5983c17df2a9cf26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2e3567bf11fdfb346689a51b: void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>),void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)> > @ 0xb7e6b -> 0x51c8d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>),void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)> @ 0x1581b3 -> 0x51c8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::prepare_frame_constants::prepare_frame_constants::closure$0
// 信息 ID: d506f66bb0b75f1f67076468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::prepare_frame_constants::prepare_frame_constants", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=392, inline_record=23536; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=392, inline_record=23700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::prepare_frame_constants::prepare_frame_constants; module=392, inline_record=23864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
